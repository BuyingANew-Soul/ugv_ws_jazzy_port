# ugv_ws — ROS2 Jazzy Port for UGV02

> **Branch:** `ros2-jazzy-dev` — this is the active development branch.
> The upstream default branch `ros2-humble-develop` is the original Waveshare code (ROS2 Humble, Docker-based). It cannot be deleted as it is the repository's default branch. Always work on and pull from `ros2-jazzy-dev`.

A port of [waveshareteam/ugv_ws](https://github.com/waveshareteam/ugv_ws) from ROS2 Humble to **ROS2 Jazzy Jalisco**, targeting the **Waveshare UGV02** with a **Raspberry Pi 5** on **Ubuntu 24.04 Noble**.

This repository is intentionally scoped to the **hardware interface layer only** — the serial bridge to the ESP32, odometry, robot description, and teleop. All higher-level robotics (sensor fusion, SLAM, navigation, manipulation) live in the companion [`ros2_stack`](https://github.com/YOUR_USERNAME/ros2_stack) repository, which is hardware-agnostic and shared across all robots in the AUWO project.

---

## Context — AUWO Project

This workspace is part of **AUWO (Autonomous Working Machines)**, a research project developing a novel architecture for autonomous mobile robots and manipulators. The UGV02 with Pi 5 serves as the primary ground robot testbed.

The architecture under development does not use Nav2 as a black box — all planning, control, and state estimation components are implemented from first principles for research validity and architectural transparency.

Related repositories in the AUWO project:

| Repository | Purpose |
|---|---|
| `ugv_ws` (this repo) | UGV02 hardware interface layer |
| `ros2_stack` | Hardware-agnostic sensor fusion, SLAM, navigation, manipulation |
| *(future)* `leo_ws` | Leo Rover hardware interface layer |
| *(future)* `auwo_arch` | Core AUWO architecture definitions and evaluation framework |

---

## Hardware

| Component | Details |
|---|---|
| Robot | Waveshare UGV02 (6-wheel 4WD) |
| Lower computer | ESP32 — General Driver for Robots board |
| Upper computer | Raspberry Pi 5 (8GB) |
| OS | Ubuntu 24.04 Noble Numbat |
| ROS2 | Jazzy Jalisco (native, no Docker) |
| Communication | UART 115200 baud, JSON protocol over `/dev/ttyAMA0` |
| Planned sensors | RPLIDAR S3 (2D), Livox Mid-360 (3D), OAK-D Pro, D435i |
| Perception compute | Jetson Orin Nano — ZED 2i, Livox, 3D SLAM |

---

## System architecture

The UGV02 uses a dual-controller design. The ESP32 handles all timing-critical low-level work. The Pi 5 handles ROS2 and all higher-level logic.

```
                    ┌─────────────────────────────────────────┐
                    │           Raspberry Pi 5                │
                    │              ROS2 Jazzy                 │
                    │                                         │
 JSON/UART          │  ugv_bringup ──→ /imu/data_raw          │
 115200 baud        │  (reader)    ──→ /odom/odom_raw         │
 /dev/ttyAMA0       │               ──→ /voltage              │
      ↕             │                                         │
 ESP32              │  ugv_bringup ←── /cmd_vel               │
 Motor PID          │  (driver)                               │
 IMU ICM20948       │                                         │
 Encoders           │  ugv_base_node ──→ /odom                │
 Battery monitor    │  (odometry)                             │
                    └─────────────────────────────────────────┘
                                      │
                              ROS2 DDS (WiFi)
                                      │
                    ┌─────────────────────────────────────────┐
                    │      Laptop / Orin Nano / ros2_stack    │
                    │   Sensor fusion · SLAM · Planning       │
                    │   Arm control · Perception · AUWO arch  │
                    └─────────────────────────────────────────┘
```

The Pi publishes standard ROS2 topics. The `ros2_stack` consumes them and publishes `/cmd_vel` back. The hardware layer has no knowledge of what generates velocity commands.

---

## What changed from upstream (Humble → Jazzy)

| Area | Change |
|---|---|
| ROS2 distro | Humble → **Jazzy**, native (no Docker) |
| `ugv_bringup.py` | Full rewrite — fixed serial-at-module-level crash, `os.walk('/')` Jetson detection, JSON buffer misalignment, `KeyError` on missing keys, `aplay` audio dependency removed |
| `ugv_driver.py` | Full rewrite — same serial issues fixed, `'T': '13'` string bug, low-battery uses ROS2 logger |
| Launch files | Fixed `get_package_share_path` removed in Jazzy, `LaunchConfigurationEquals` removed in Jazzy |
| `ugv_base_node` CMakeLists | Fixed `ament_target_dependencies` before `add_executable`, added missing `std_msgs` and `tf2_geometry_msgs` |
| Removed: SLAM | `cartographer`, `gmapping` deleted — no Jazzy binaries |
| Removed: web/AI/sim | `ugv_gazebo`, `ugv_chat_ai`, `ugv_web_app`, `ugv_vision` removed |
| Removed: vendors | `vizanti`, `emcl2_ros2`, `costmap_converter`, `ldlidar`, `rf2o_laser_odometry`, `teb_local_planner` removed — available via `apt` or replaced in `ros2_stack` |

---

## Repository structure

```
src/
├── ugv_main/
│   ├── ugv_bringup/          ESP32 serial bridge (reader + driver nodes)
│   ├── ugv_base_node/        Differential drive odometry (C++)
│   ├── ugv_interface/        Custom ROS2 interfaces (Behavior.action, MapSave.srv)
│   ├── ugv_description/      URDF models + meshes (ugv_rover, ugv_beast, rasp_rover)
│   └── ugv_tools/            Keyboard ctrl, joystick ctrl, behavior ctrl
└── ugv_else/
    └── robot_pose_publisher/ Publishes /robot_pose (Pose) from TF tree
```

---

## Installation

### ROS2 Jazzy

```bash
sudo apt install -y ros-jazzy-desktop python3-colcon-common-extensions
```

### Dependencies

```bash
sudo apt install -y \
  ros-jazzy-robot-localization \
  ros-jazzy-imu-complementary-filter \
  ros-jazzy-teleop-twist-keyboard \
  ros-jazzy-teleop-twist-joy \
  ros-jazzy-joy \
  python3-serial
```

### Clone — use the jazzy branch explicitly

```bash
git clone -b ros2-jazzy-dev https://github.com/YOUR_USERNAME/ugv_ws.git \
  ~/ugv_ws_jazzy_port
cd ~/ugv_ws_jazzy_port
```

### Build

```bash
# Round 1 — interfaces and C++ nodes
colcon build --packages-select \
  ugv_interface ugv_base_node robot_pose_publisher

# Round 2 — Python packages
colcon build --packages-select \
  ugv_bringup ugv_description ugv_tools \
  --symlink-install
```

### Environment — add to `~/.bashrc`

```bash
source /opt/ros/jazzy/setup.bash
source ~/ugv_ws_jazzy_port/install/setup.bash
export ROS_DOMAIN_ID=42          # match on all machines
export UGV_MODEL=ugv_rover       # UGV02 uses the ugv_rover URDF
```

---

## Hardware connection

### Pi 5 UART setup

Add to `/boot/firmware/config.txt`:
```ini
enable_uart=1
dtoverlay=uart0-pi5
```

Remove `console=serial0,115200` from `/boot/firmware/cmdline.txt` — the file must remain one single line.

```bash
sudo usermod -aG dialout $USER
```

### 40-pin header wiring

| Signal | UGV02 pin | Pi 5 pin |
|---|---|---|
| 5V power | 2, 4 | 2, 4 |
| GND | 6 | 6 |
| ESP32 TX → Pi RX | 8 | 10 (GPIO15) |
| Pi TX → ESP32 RX | 10 | 8 (GPIO14) |

> **Pi 5 power warning:** Measure pins 2/4 against GND with a multimeter before connecting. Must read 4.75–5.25V. If uncertain, power Pi via USB-C independently and connect only GND + TX + RX from the header.

### Verify serial

```bash
python3 -c "
import serial
s = serial.Serial('/dev/ttyAMA0', 115200, timeout=2)
print(s.readline().decode('utf-8', errors='replace').strip())
s.close()
"
# Expected: {"T":1001,"L":0,"R":0,...,"v":1199}
```

---

## Usage

### Start the robot

```bash
ros2 launch ugv_bringup bringup_lidar.launch.py
```

Optional args: `use_rviz:=true`, `pub_odom_tf:=true`

### With EKF fusion

```bash
ros2 launch ugv_bringup bringup_imu_ekf.launch.py
```

### Teleop

```bash
# Keyboard — always set reduced speed for indoor use
ros2 run teleop_twist_keyboard teleop_twist_keyboard \
  --ros-args -p speed:=0.2 -p turn:=0.5

# Joystick
ros2 launch ugv_tools teleop_twist_joy.launch.py
```

### Verify topics

```bash
ros2 topic hz /imu/data_raw      # ~100 Hz
ros2 topic hz /odom              # ~10 Hz
ros2 topic echo /voltage         # battery in volts
```

### Behavior actions

```bash
ros2 run ugv_tools behavior_ctrl

# Drive forward 0.2 m
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"drive_on_heading\", \"data\": 0.2}]'}"

# Rotate 90° (positive = left)
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"spin\", \"data\": 90}]'}"

# Stop
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"stop\", \"data\": 0}]'}"
```

### LED control

```bash
# data[0] = IO4, data[1] = IO5, range 0–255
ros2 topic pub /ugv/led_ctrl std_msgs/msg/Float32MultiArray "{data: [128, 0]}" -1
```

---

## Published topics

| Topic | Type | Rate | Source |
|---|---|---|---|
| `/imu/data_raw` | `sensor_msgs/Imu` | ~100 Hz | `ugv_bringup` |
| `/imu/mag` | `sensor_msgs/MagneticField` | ~100 Hz | `ugv_bringup` |
| `/odom/odom_raw` | `std_msgs/Float32MultiArray` | ~100 Hz | `ugv_bringup` |
| `/voltage` | `std_msgs/Float32` | ~100 Hz | `ugv_bringup` |
| `/odom` | `nav_msgs/Odometry` | ~10 Hz | `ugv_base_node` |
| `/tf` | `tf2_msgs/TFMessage` | continuous | `ugv_base_node` + `robot_state_publisher` |
| `/robot_pose` | `geometry_msgs/Pose` | ~10 Hz | `robot_pose_publisher` |

## Subscribed topics

| Topic | Type | Node |
|---|---|---|
| `/cmd_vel` | `geometry_msgs/Twist` | `ugv_driver` |
| `/ugv/joint_states` | `sensor_msgs/JointState` | `ugv_driver` (pan-tilt) |
| `/ugv/led_ctrl` | `std_msgs/Float32MultiArray` | `ugv_driver` |

---

## Multi-machine ROS2

```bash
# Same on all machines
export ROS_DOMAIN_ID=42
```

Test multicast:
```bash
ros2 multicast receive &   # laptop
ros2 multicast send        # Pi
```

If multicast is blocked, configure FastDDS unicast with the Pi's static IP — see [FastDDS documentation](https://fast-dds.docs.eprosima.com/en/latest/).

---

## Known issues

**JSON decode errors on launch** — Normal. The ESP32 broadcasts at ~100 Hz and the buffer occasionally contains merged packets on first connection. Clears within one second. The node extracts JSON between the outermost `{` and `}` and validates all required keys before publishing.

**`rclpy.RCLError: failed to shutdown`** — Benign. Printed on Ctrl+C when the context is already shutting down.

**`robot_state_publisher` URDF warning** — Jazzy deprecation notice for passing URDF as a command-line argument. Will be migrated to the `robot_description` parameter.

**Teleop speed** — `teleop_twist_keyboard` defaults to 0.5 m/s. Always launch with `--ros-args -p speed:=0.2`.

---

## Upstream references

| Resource | Link |
|---|---|
| Original workspace | [waveshareteam/ugv_ws](https://github.com/waveshareteam/ugv_ws) |
| ESP32 firmware | [waveshareteam/ugv_base_ros](https://github.com/waveshareteam/ugv_base_ros) |
| Non-ROS Pi app | [waveshareteam/ugv_rpi](https://github.com/waveshareteam/ugv_rpi) |
| Product wiki | [waveshare.com/wiki/UGV02](https://www.waveshare.com/wiki/UGV02) |