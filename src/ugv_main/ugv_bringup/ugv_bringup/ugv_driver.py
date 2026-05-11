#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from sensor_msgs.msg import JointState
from std_msgs.msg import Float32, Float32MultiArray
import serial
import json
import os


def get_serial_port() -> str:
    """Return the correct UART device without walking the filesystem."""
    if os.path.exists('/etc/nv_tegra_release'):
        return '/dev/ttyTHS1'
    return '/dev/ttyAMA0'


class UgvDriver(Node):
    """ROS2 node: translates ROS commands → JSON and sends to ESP32 via UART."""

    def __init__(self):
        super().__init__('ugv_driver')

        # Open serial port inside __init__, not at module level
        port = get_serial_port()
        self.get_logger().info(f'UgvDriver opening serial: {port}')
        self.ser = serial.Serial(port, 115200, timeout=1)

        # Subscriptions
        self.cmd_vel_sub = self.create_subscription(
            Twist, 'cmd_vel', self._cmd_vel_cb, 10)
        self.joint_states_sub = self.create_subscription(
            JointState, 'ugv/joint_states', self._joint_states_cb, 10)
        self.led_ctrl_sub = self.create_subscription(
            Float32MultiArray, 'ugv/led_ctrl', self._led_ctrl_cb, 10)
        self.voltage_sub = self.create_subscription(
            Float32, 'voltage', self._voltage_cb, 10)

    def _send(self, data: dict):
        """Serialise dict to JSON and write to UART."""
        try:
            self.ser.write((json.dumps(data) + '\n').encode())
        except Exception as e:
            self.get_logger().error(f'Serial write error: {e}')

    def _cmd_vel_cb(self, msg: Twist):
        linear = msg.linear.x
        angular = msg.angular.z

        # Enforce minimum angular velocity when not moving linearly
        # (prevents motor stall at very low angular commands)
        if linear == 0.0:
            if 0.0 < angular < 0.2:
                angular = 0.2
            elif -0.2 < angular < 0.0:
                angular = -0.2

        # T must be integer, not string — original bug fixed here
        self._send({'T': 13, 'X': linear, 'Z': angular})

    def _joint_states_cb(self, msg: JointState):
        try:
            x_rad = msg.position[msg.name.index('pt_base_link_to_pt_link1')]
            y_rad = msg.position[msg.name.index('pt_link1_to_pt_link2')]
        except ValueError:
            self.get_logger().warning('Joint names not found in JointState msg')
            return

        x_deg = (180.0 * x_rad) / 3.14159265
        y_deg = (180.0 * y_rad) / 3.14159265
        self._send({'T': 134, 'X': x_deg, 'Y': y_deg, 'SX': 600, 'SY': 600})

    def _led_ctrl_cb(self, msg: Float32MultiArray):
        if len(msg.data) < 2:
            return
        self._send({'T': 132, 'IO4': msg.data[0], 'IO5': msg.data[1]})

    def _voltage_cb(self, msg: Float32):
        # Low battery threshold: below 9.0V on a 3S pack = cells below 3.0V each
        if 0.1 < msg.data < 9.0:
            self.get_logger().warn(
                f'LOW BATTERY: {msg.data:.2f}V — please charge soon!',
                throttle_duration_sec=30.0  # warn at most once every 30s
            )

    def destroy_node(self):
        if self.ser.is_open:
            self.ser.close()
        super().destroy_node()


def main(args=None):
    rclpy.init(args=args)
    node = UgvDriver()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():          # ← guard against double-shutdown
            rclpy.shutdown()


if __name__ == '__main__':
    main()