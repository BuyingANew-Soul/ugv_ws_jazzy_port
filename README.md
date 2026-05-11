# ugv_ws — ROS2 Jazzy Port

A port of [waveshareteam/ugv_ws](https://github.com/waveshareteam/ugv_ws) from ROS2 Humble to **ROS2 Jazzy Jalisco**, targeting the **Waveshare UGV02** with a **Raspberry Pi 5** on **Ubuntu 24.04 Noble**.

The original workspace ran inside a Docker container on Raspberry Pi OS with ROS2 Humble. This fork runs natively on Ubuntu 24.04 with no Docker required. SLAM packages (cartographer, gmapping) have been removed and will be replaced with slam_toolbox in a future update. RTABMap is preserved.

---

## Hardware

| Component | Details |
|---|---|
| Robot | Waveshare UGV02 (6-wheel 4WD) |
| Lower computer | ESP32 via General Driver for Robots board |
| Upper computer | Raspberry Pi 5 (8GB) |
| OS | Ubuntu 24.04 Noble Numbat |
| ROS2 | Jazzy Jalisco (native, no Docker) |
| Communication | UART 115200 baud, JSON protocol |

---

## Architecture

The UGV02 uses a dual-controller design:

- **ESP32 (lower computer):** runs motor PID, reads IMU (ICM20948), wheel encoders, battery voltage. Broadcasts sensor data as JSON at ~100 Hz over UART. Accepts movement commands as JSON (e.g. `{"T":13,"X":0.5,"Z":0.0}`).
- **Raspberry Pi 5 (upper computer):** runs ROS2. Bridges the ESP32 JSON protocol to standard ROS2 topics. Handles navigation, SLAM, and any higher-level autonomy.

```
ESP32  ←──── JSON/UART 115200 ────→  Raspberry Pi 5
             /dev/ttyAMA0                ROS2 Jazzy
                                     ┌──────────────────────┐
                                     │ ugv_bringup (reader) │ → /imu/data_raw
                                     │ ugv_bringup (driver) │ ← /cmd_vel
                                     │ ugv_base_node        │ → /odom
                                     └──────────────────────┘
```

---

## What changed from upstream

| Area | Change |
|---|---|
| ROS2 distro | Humble → **Jazzy** |
| Serial node | Rewrote `ugv_bringup.py` and `ugv_driver.py` — fixed serial-at-module-level crash, Jetson detection (`os.walk('/')` → `os.path.exists`), JSON buffer misalignment, missing key crashes, audio dependency |
| Launch files | Fixed `get_package_share_path` (removed in Jazzy), `LaunchConfigurationEquals` (removed in Jazzy) |
| Nav2 params | Rewrote all four YAML files — removed `cartographer_ros_msgs` service references |
| SLAM | Removed `cartographer` and `gmapping` packages entirely — no Jazzy binaries exist |
| Gazebo | `ugv_gazebo` left in place but not built — requires full Gazebo Harmonic rewrite |
| CMakeLists | Fixed `ament_target_dependencies` called before `add_executable` |
| `ugv_base_node` | Added missing `tf2_geometry_msgs` and `std_msgs` dependencies |

---

## Hardware connection — 40-pin UART

The Pi 5 connects to the UGV02 driver board via the 40-pin GPIO header. The driver board's DC-DC 5V regulator powers the Pi through this header.

> **Warning for Pi 5:** The Pi 5 can draw up to 5A under load. Verify the UGV02's 5V output is within 4.75–5.25V with a multimeter before connecting. If the regulator output is marginal, power the Pi via USB-C separately and use only pins 6, 8, and 10 for UART signals.

| Signal | UGV02 pin | Pi 5 pin |
|---|---|---|
| 5V power | Pin 2, 4 | Pin 2, 4 |
| GND | Pin 6 | Pin 6 |
| ESP32 TX → Pi RX | Pin 8 | Pin 10 (GPIO15) |
| Pi TX → ESP32 RX | Pin 10 | Pin 8 (GPIO14) |

All signal pins are 3.3V logic — no level shifter needed.

### Pi 5 UART configuration

Add to `/boot/firmware/config.txt`:
```
enable_uart=1
dtoverlay=uart0-pi5
```

In `/boot/firmware/cmdline.txt`, remove `console=serial0,115200` if present. The file must remain a single line.

```bash
# Verify serial port after reboot
ls -la /dev/ttyAMA0

# Add user to dialout group
sudo usermod -aG dialout $USER
```

### Quick serial test (robot powered on, Pi connected)

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

## Installation

### 1. System requirements

Ubuntu 24.04 Noble Numbat on Raspberry Pi 5.

### 2. Install ROS2 Jazzy

```bash
sudo apt update && sudo apt install -y software-properties-common curl gnupg
sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
  -o /usr/share/keyrings/ros-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] \
  http://packages.ros.org/ros2/ubuntu $(lsb_release -cs) main" \
  | sudo tee /etc/apt/sources.list.d/ros2.list > /dev/null
sudo apt update
sudo apt install -y ros-jazzy-desktop python3-colcon-common-extensions python3-rosdep
```

### 3. Install ROS2 dependencies

```bash
sudo apt install -y \
  ros-jazzy-nav2-bringup \
  ros-jazzy-navigation2 \
  ros-jazzy-robot-localization \
  ros-jazzy-imu-tools \
  ros-jazzy-imu-complementary-filter \
  ros-jazzy-imu-filter-madgwick \
  ros-jazzy-teleop-twist-keyboard \
  ros-jazzy-teleop-twist-joy \
  ros-jazzy-joy \
  ros-jazzy-rtabmap-ros \
  python3-serial
```

### 4. Clone and build

```bash
git clone https://github.com/<your-fork>/ugv_ws_jazzy_port.git ~/ugv_ws_jazzy_port
cd ~/ugv_ws_jazzy_port

# Round 1 — core packages
colcon build --packages-select \
  ugv_interface \
  ugv_base_node \
  robot_pose_publisher

# Round 2 — bringup (serial bridge)
colcon build --packages-select ugv_bringup --symlink-install

# Round 3 — everything else
colcon build --packages-select \
  ugv_description \
  ugv_tools \
  ugv_nav \
  ugv_slam \
  ugv_vision \
  --symlink-install
```

### 5. Environment setup

Add to `~/.bashrc`:

```bash
source /opt/ros/jazzy/setup.bash
source ~/ugv_ws_jazzy_port/install/setup.bash
export ROS_DOMAIN_ID=42         # match this on all machines in your network
export UGV_MODEL=ugv_rover      # UGV02 uses the ugv_rover URDF
```

Available `UGV_MODEL` values: `ugv_rover`, `ugv_beast`, `rasp_rover`.

---

## Usage

### Start the robot

```bash
ros2 launch ugv_bringup bringup_lidar.launch.py
```

This starts: `robot_state_publisher`, `joint_state_publisher`, `ugv_bringup` (serial reader), `ugv_driver` (cmd_vel writer), `ugv_base_node` (odometry).

### Keyboard teleoperation

```bash
# In a second terminal
ros2 run teleop_twist_keyboard teleop_twist_keyboard \
  --ros-args -p speed:=0.2 -p turn:=0.5
```

Speed adjustment keys: `q/z` scale both, `w/x` linear only, `e/c` angular only. Press `s` or space to stop.

### With EKF odometry fusion

```bash
ros2 launch ugv_bringup bringup_imu_ekf.launch.py
```

### Check topics

```bash
ros2 topic list
ros2 topic hz /imu/data_raw      # ~100 Hz
ros2 topic hz /odom              # ~10 Hz
ros2 topic echo /voltage         # battery voltage in volts
```

### View robot model in RViz

```bash
ros2 launch ugv_description display.launch.py use_rviz:=true
```

### Joystick control

```bash
ros2 launch ugv_tools teleop_twist_joy.launch.py
```

### Behavior control (action interface)

```bash
ros2 run ugv_tools behavior_ctrl
```

Drive forward 0.1 m:
```bash
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"drive_on_heading\", \"data\": 0.1}]'}"
```

Rotate 90°:
```bash
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"spin\", \"data\": 90}]'}"
```

Stop:
```bash
ros2 action send_goal /behavior ugv_interface/action/Behavior \
  "{command: '[{\"T\": 1, \"type\": \"stop\", \"data\": 0}]'}"
```

### 3D mapping with RTABMap

```bash
# RTABMap visualizer
ros2 launch ugv_slam rtabmap_rgbd.launch.py use_rviz:=false

# Or with RViz
ros2 launch ugv_slam rtabmap_rgbd.launch.py use_rviz:=true
```

Map is saved automatically to `~/.ros/rtabmap.db` on Ctrl+C.

### Navigation (2D)

Requires a pre-built map. Using AMCL localization:

```bash
ros2 launch ugv_nav nav.launch.py use_localization:=amcl use_rviz:=true
```

Options:
- `use_localization`: `amcl` (default) or `emcl`
- `use_localplan`: `teb` (default) or `dwa`
- `map`: path to map YAML file

After launch, set initial pose in RViz using "2D Pose Estimate", then use "Nav2 Goal" to send navigation targets.

### Save a map

```bash
cd src/ugv_main/ugv_nav/maps
ros2 run nav2_map_server map_saver_cli -f ./map
```

### LED control

```bash
# data[0] = IO4 (near OAK camera), data[1] = IO5 (near USB camera), range 0–255
ros2 topic pub /ugv/led_ctrl std_msgs/msg/Float32MultiArray "{data: [128, 0]}" -1
```

### Get robot pose

```bash
ros2 topic echo /robot_pose --once
```

---

## Multi-machine ROS2 (laptop + robot)

Connect both machines to the same network. Add to `~/.bashrc` on both:

```bash
export ROS_DOMAIN_ID=42
```

ROS2 DDS handles discovery automatically. Run `rviz2` on your laptop — it will see all topics from the Pi with no further configuration.

Test multicast is working:
```bash
# On laptop
ros2 multicast receive &
# On Pi
ros2 multicast send
```

If multicast is blocked by your router, create `~/fastdds_unicast.xml` with the Pi's IP and set `export FASTRTPS_DEFAULT_PROFILES_FILE=~/fastdds_unicast.xml` on both machines.

---

## Package reference

### ugv_main (active)

| Package | Purpose |
|---|---|
| `ugv_bringup` | Serial bridge to ESP32. Publishes `/imu/data_raw`, `/imu/mag`, `/odom/odom_raw`, `/voltage`. Subscribes to `/cmd_vel`. |
| `ugv_base_node` | Differential drive kinematics. Computes `/odom` from wheel encoder data. |
| `ugv_interface` | Custom action/message definitions (`Behavior` action). |
| `ugv_description` | URDF models, `robot_state_publisher` launch. |
| `ugv_tools` | Keyboard ctrl, joystick ctrl, behavior controller. |
| `ugv_nav` | Nav2 launch wrappers with AMCL/EMCL localization. |
| `ugv_slam` | RTABMap SLAM launch wrappers. |
| `ugv_vision` | AprilTag detection and tracking nodes. |

### ugv_main (deferred / not built)

| Package | Status |
|---|---|
| `ugv_gazebo` | Requires full rewrite for Gazebo Harmonic (Classic was dropped in Jazzy). |
| `ugv_web_app` | Not yet ported. |
| `ugv_chat_ai` | Not yet ported. |

### ugv_else (active)

| Package | Purpose |
|---|---|
| `robot_pose_publisher` | Publishes `/robot_pose` (geometry_msgs/Pose) from TF. |
| `emcl2_ros2` | Monte-Carlo localization alternative to AMCL. |
| `teb_local_planner` | TEB local planner for Nav2. |
| `rf2o_laser_odometry` | Laser scan odometry (used with LiDAR). |
| `explore_lite` | Frontier exploration for autonomous mapping. |
| `apriltag_ros` | AprilTag detection. |
| `vizanti` | Web-based RViz alternative. |

### ugv_else (removed)

| Package | Reason |
|---|---|
| `cartographer` | No ROS2 Jazzy binary exists. Replaced by slam_toolbox (planned). |
| `gmapping` | Superseded. No Jazzy support. |
| `ldlidar` | Removed from active build — missing `pthread.h` include on GCC 13. Re-add when LiDAR is needed. |

---

## Known issues

**JSON decode errors on startup** — The ESP32 broadcasts at ~100 Hz. The serial buffer occasionally contains merged or partial packets. The `ugv_bringup` node handles this by extracting JSON between the first `{` and last `}` on each line and validating required keys before publishing. Errors at launch are normal and resolve within a second.

**`rclpy.RCLError: failed to shutdown`** — Benign warning printed when Ctrl+C is pressed. Occurs because the ROS2 context is already shutting down when the `finally` block runs. Does not indicate a real problem.

**`robot_state_publisher` URDF warning** — `No robot_description parameter, but command-line argument available` is a Jazzy deprecation notice. The URDF path is passed as an argument for now; migration to a parameter is planned.

**Teleop speed** — Default `teleop_twist_keyboard` speed (0.5 m/s) is high for the UGV02. Always launch with reduced defaults: `--ros-args -p speed:=0.2 -p turn:=0.5`.

---

## Upstream

- Original repo: [waveshareteam/ugv_ws](https://github.com/waveshareteam/ugv_ws)
- ESP32 firmware: [waveshareteam/ugv_base_ros](https://github.com/waveshareteam/ugv_base_ros)
- Non-ROS Pi app: [waveshareteam/ugv_rpi](https://github.com/waveshareteam/ugv_rpi)
- Product wiki: [waveshare.com/wiki/UGV02](https://www.waveshare.com/wiki/UGV02)