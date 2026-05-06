import os
from launch import LaunchDescription
from launch.substitutions import LaunchConfiguration, EqualsSubstitution
from launch.conditions import IfCondition
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
from launch.actions import DeclareLaunchArgument, OpaqueFunction
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource


def launch_setup(context, *args, **kwargs):
    use_localization = context.launch_configurations['use_localization']

    ugv_nav_dir = get_package_share_directory('ugv_nav')
    nav2_bringup_dir = get_package_share_directory('nav2_bringup')
    emcl_dir = get_package_share_directory('emcl2')

    # Parameter file selection: amcl or emcl, combined with local planner
    use_localplan = context.launch_configurations['use_localplan']
    config_map = {
        ('amcl', 'teb'): 'amcl_teb.yaml',
        ('amcl', 'dwa'): 'amcl_dwa.yaml',
        ('emcl', 'teb'): 'emcl_teb.yaml',
        ('emcl', 'dwa'): 'emcl_dwa.yaml',
    }
    param_filename = config_map.get(
        (use_localization, use_localplan), 'amcl_teb.yaml')
    param_file = os.path.join(ugv_nav_dir, 'param', param_filename)

    map_yaml_path = LaunchConfiguration(
        'map', default=os.path.join(ugv_nav_dir, 'maps', 'map.yaml'))

    bringup_lidar_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(os.path.join(
            get_package_share_directory('ugv_bringup'), 'launch', 'bringup_lidar.launch.py')),
        launch_arguments={
            'use_rviz': LaunchConfiguration('use_rviz'),
            'rviz_config': 'nav_2d',
        }.items()
    )

    # AMCL branch — IfCondition + EqualsSubstitution (replaces LaunchConfigurationEquals)
    nav2_amcl_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(nav2_bringup_dir, 'launch', 'bringup_launch.py')),
        launch_arguments={'map': map_yaml_path, 'params_file': param_file}.items(),
        condition=IfCondition(EqualsSubstitution(
            LaunchConfiguration('use_localization'), 'amcl'))
    )

    # EMCL branch
    nav2_emcl_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(os.path.join(
            ugv_nav_dir, 'launch/nav_bringup', 'nav2_bringup.launch.py')),
        launch_arguments={'map': map_yaml_path, 'params_file': param_file}.items(),
        condition=IfCondition(EqualsSubstitution(
            LaunchConfiguration('use_localization'), 'emcl'))
    )

    emcl_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(emcl_dir, 'launch', 'emcl2.launch.py')),
        launch_arguments={
            'params_file': os.path.join(emcl_dir, 'config', 'emcl2_quick_start.param.yaml'),
        }.items(),
        condition=IfCondition(EqualsSubstitution(
            LaunchConfiguration('use_localization'), 'emcl'))
    )

    robot_pose_publisher_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(os.path.join(
            get_package_share_directory('robot_pose_publisher'),
            'launch', 'robot_pose_publisher_launch.py'))
    )

    return [
        bringup_lidar_launch,
        nav2_amcl_launch,
        nav2_emcl_launch,
        emcl_launch,
        robot_pose_publisher_launch,
    ]


def generate_launch_description():
    return LaunchDescription([
        DeclareLaunchArgument(
            'use_localplan', default_value='teb',
            description='Local planner: dwa or teb'),
        DeclareLaunchArgument(
            'use_localization', default_value='amcl',
            description='Localization: amcl or emcl'),
        DeclareLaunchArgument(
            'use_rviz', default_value='false',
            description='Launch RViz2'),
        DeclareLaunchArgument(
            'map', default_value='',
            description='Path to map yaml'),
        OpaqueFunction(function=launch_setup),
    ])