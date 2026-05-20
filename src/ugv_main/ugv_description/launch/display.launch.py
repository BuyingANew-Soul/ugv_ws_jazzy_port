import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.conditions import IfCondition
from launch.substitutions import (
    Command, LaunchConfiguration,
    PathJoinSubstitution, EnvironmentVariable
)
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def generate_launch_description():

    desc_pkg = get_package_share_directory('ugv_description')

    # UGV_MODEL env var selects which URDF to load
    # Default to ugv_rover if not set
    ugv_model = os.environ.get('UGV_MODEL', 'ugv_rover')
    urdf_path = os.path.join(desc_pkg, 'urdf', f'{ugv_model}.urdf')

    if not os.path.exists(urdf_path):
        raise RuntimeError(
            f"URDF not found at {urdf_path}\n"
            f"UGV_MODEL='{ugv_model}' — valid values: ugv_rover, ugv_beast, rasp_rover"
        )

    with open(urdf_path, 'r') as f:
        robot_description = f.read()

    use_rviz_arg = DeclareLaunchArgument(
        'use_rviz', default_value='true',
        description='Launch RViz2'
    )
    rviz_config_arg = DeclareLaunchArgument(
        'rviz_config', default_value='',
        description='Path to RViz config file (optional)'
    )

    # Default RViz config — falls back to blank config if none exists
    default_rviz = os.path.join(desc_pkg, 'rviz', 'display.rviz')
    rviz_config = default_rviz if os.path.exists(default_rviz) else ''

    robot_state_publisher = Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        name='robot_state_publisher',
        namespace='ugv',
        output='screen',
        parameters=[{
            'robot_description': robot_description,
            'publish_frequency': 50.0,
        }]
    )

    joint_state_publisher_gui = Node(
        package='joint_state_publisher_gui',
        executable='joint_state_publisher_gui',
        name='joint_state_publisher_gui',
        namespace='ugv',
        output='screen',
    )

    rviz2 = Node(
        package='rviz2',
        executable='rviz2',
        name='rviz2',
        output='screen',
        arguments=['-d', rviz_config] if rviz_config else [],
        condition=IfCondition(LaunchConfiguration('use_rviz')),
    )

    return LaunchDescription([
        use_rviz_arg,
        rviz_config_arg,
        robot_state_publisher,
        joint_state_publisher_gui,
        rviz2,
    ])