#!/usr/bin/env python3
import serial
import json
import queue
import threading
import rclpy
from rclpy.node import Node
import logging
import time
import os
from std_msgs.msg import Header, Float32MultiArray, Float32
from sensor_msgs.msg import Imu, MagneticField


def get_serial_port():
    """Detect the correct serial port without walking the entire filesystem."""
    # Jetson: /etc/nv_tegra_release exists
    if os.path.exists('/etc/nv_tegra_release'):
        return '/dev/ttyTHS1'
    # Raspberry Pi: use GPIO UART
    return '/dev/ttyAMA0'


class ReadLine:
    """Efficient line reader for serial port — avoids Python readline() blocking."""
    def __init__(self, s):
        self.buf = bytearray()
        self.s = s

    def readline(self):
        i = self.buf.find(b"\n")
        if i >= 0:
            r = self.buf[:i + 1]
            self.buf = self.buf[i + 1:]
            return r
        while True:
            i = max(1, min(512, self.s.in_waiting))
            data = self.s.read(i)
            i = data.find(b"\n")
            if i >= 0:
                r = self.buf + data[:i + 1]
                self.buf[0:] = data[i + 1:]
                return r
            else:
                self.buf.extend(data)

    def clear_buffer(self):
        self.s.reset_input_buffer()


class BaseController:
    """Manages UART communication with the ESP32 lower computer."""

    def __init__(self, uart_dev, baud):
        self.logger = logging.getLogger('BaseController')
        self.ser = serial.Serial(uart_dev, baud, timeout=1)
        self.rl = ReadLine(self.ser)
        self.command_queue = queue.Queue()
        self.command_thread = threading.Thread(
            target=self._process_commands, daemon=True)
        self.command_thread.start()
        # Default data structure matching ESP32 T=1001 feedback packet
        self.base_data = {
            "T": 0, "L": 0, "R": 0,
            "ax": 0, "ay": 0, "az": 0,
            "gx": 0, "gy": 0, "gz": 0,
            "mx": 0, "my": 0, "mz": 0,
            "odl": 0, "odr": 0, "v": 0
        }
    def feedback_data(self):
        """Read UART and extract first valid JSON object found."""
        try:
            line = self.rl.readline().decode('utf-8', errors='ignore')
            # Find the first { and last } to extract the JSON object
            start = line.find('{')
            end = line.rfind('}')
            if start == -1 or end == -1 or end <= start:
                return None
            json_str = line[start:end + 1]
            parsed = json.loads(json_str)
            self.base_data = parsed
            return parsed
        except json.JSONDecodeError:
            self.rl.clear_buffer()
            return None
        except Exception as e:
            self.get_logger().error(f'feedback_data error: {e}') if hasattr(self, 'get_logger') else None
            self.rl.clear_buffer()
            return None

    def send_command(self, data: dict):
        """Queue a JSON command to be sent to the ESP32."""
        self.command_queue.put(data)

    def _process_commands(self):
        """Worker thread: drain command queue and write to UART."""
        while True:
            data = self.command_queue.get()
            try:
                self.ser.write((json.dumps(data) + '\n').encode('utf-8'))
            except Exception as e:
                self.logger.error(f"Serial write error: {e}")

    def base_json_ctrl(self, input_json: dict):
        self.send_command(input_json)

    def close(self):
        if self.ser.is_open:
            self.ser.close()


class UgvBringup(Node):
    """ROS2 node: reads ESP32 sensor feedback and publishes to ROS topics."""
    

    def __init__(self):
        super().__init__('ugv_bringup')
        self.REQUIRED_KEYS = {'T', 'L', 'R', 'ax', 'ay', 'az', 'gx', 'gy', 'gz',
                 'mx', 'my', 'mz', 'odl', 'odr', 'v'}

        # Publishers
        self.imu_raw_pub = self.create_publisher(Imu, 'imu/data_raw', 100)
        self.imu_mag_pub = self.create_publisher(MagneticField, 'imu/mag', 100)
        self.odom_raw_pub = self.create_publisher(
            Float32MultiArray, 'odom/odom_raw', 100)
        self.voltage_pub = self.create_publisher(Float32, 'voltage', 50)

        # Serial bridge — opened here, inside the node, not at module level
        serial_port = get_serial_port()
        self.get_logger().info(f'Opening serial port: {serial_port}')
        self.base_ctrl = BaseController(serial_port, 115200)

        # 1 kHz feedback loop
        self.feedback_timer = self.create_timer(0.001, self._feedback_loop)

    
    def _feedback_loop(self):
        data = self.base_ctrl.feedback_data()
        if data is not None and data.get('T') == 1001:
            if self.REQUIRED_KEYS.issubset(data.keys()):
                self._publish_imu_raw(data)
                self._publish_imu_mag(data)
                self._publish_odom_raw(data)
                self._publish_voltage(data)

    def _publish_imu_raw(self, d):
        msg = Imu()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'base_imu_link'
        # ICM20948: accel scale 8192 LSB/g, gyro scale 16.4 LSB/deg/s
        msg.linear_acceleration.x = 9.8 * float(d['ax']) / 8192.0
        msg.linear_acceleration.y = 9.8 * float(d['ay']) / 8192.0
        msg.linear_acceleration.z = 9.8 * float(d['az']) / 8192.0
        msg.angular_velocity.x = 3.14159265 * float(d['gx']) / (16.4 * 180.0)
        msg.angular_velocity.y = 3.14159265 * float(d['gy']) / (16.4 * 180.0)
        msg.angular_velocity.z = 3.14159265 * float(d['gz']) / (16.4 * 180.0)
        self.imu_raw_pub.publish(msg)

    def _publish_imu_mag(self, d):
        msg = MagneticField()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'base_imu_link'
        # ICM20948 magnetometer: 0.15 μT/LSB
        msg.magnetic_field.x = float(d['mx']) * 0.15
        msg.magnetic_field.y = float(d['my']) * 0.15
        msg.magnetic_field.z = float(d['mz']) * 0.15
        self.imu_mag_pub.publish(msg)

    def _publish_odom_raw(self, d):
        # odl/odr are in centimetres from ESP32 → convert to metres
        msg = Float32MultiArray(data=[
            float(d['odl']) / 100.0,
            float(d['odr']) / 100.0
        ])
        self.odom_raw_pub.publish(msg)

    def _publish_voltage(self, d):
        msg = Float32()
        msg.data = float(d['v']) / 100.0
        self.voltage_pub.publish(msg)

    def destroy_node(self):
        self.base_ctrl.close()
        super().destroy_node()


def main(args=None):
    rclpy.init(args=args)
    node = UgvBringup()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()