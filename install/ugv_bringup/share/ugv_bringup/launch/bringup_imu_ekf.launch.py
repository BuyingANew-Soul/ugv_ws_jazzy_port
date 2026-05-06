import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    pub_odom_tf_arg = DeclareLaunchArgument(
        'pub_odom_tf', default_value='false',
        description='Publish odom → base_footprint tf (false when EKF publishes it)')
    use_rviz_arg = DeclareLaunchArgument(
        'use_rviz', default_value='false',
        description='Launch RViz2')
    rviz_config_arg = DeclareLaunchArgument(
        'rviz_config', default_value='bringup',
        description='RViz config name')

    robot_state_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(os.path.join(
            get_package_share_directory('ugv_description'), 'launch', 'display.launch.py')),
        launch_arguments={
            'use_rviz': LaunchConfiguration('use_rviz'),
            'rviz_config': LaunchConfiguration('rviz_config'),
        }.items()
    )

    bringup_node = Node(package='ugv_bringup', executable='ugv_bringup')
    driver_node = Node(package='ugv_bringup', executable='ugv_driver')

    imu_complementary_filter_node = Node(
        package='imu_complementary_filter',
        executable='complementary_filter_node',
        name='complementary_filter_gain_node',
        output='screen',
        parameters=[{
            'do_bias_estimation': True,
            'do_adaptive_gain': True,
            'use_mag': False,
            'gain_acc': 0.01,
            'gain_mag': 0.01,
        }]
    )

    # laser_bringup_launch = IncludeLaunchDescription(
    #     PythonLaunchDescriptionSource(os.path.join(
    #         get_package_share_directory('ldlidar'), 'launch', 'ldlidar.launch.py'))
    # )

    base_node = Node(
        package='ugv_base_node',
        executable='base_node_ekf',
        parameters=[{'pub_odom_tf': LaunchConfiguration('pub_odom_tf')}]
    )

    ekf_node = Node(
        package='robot_localization',
        executable='ekf_node',
        name='ekf_filter_node',
        output='screen',
        parameters=[os.path.join(
            get_package_share_directory('ugv_bringup'), 'param', 'ekf.yaml')],
        remappings=[('/odometry/filtered', '/odom')]
    )

    return LaunchDescription([
        pub_odom_tf_arg,
        use_rviz_arg,
        rviz_config_arg,
        robot_state_launch,
        bringup_node,
        imu_complementary_filter_node,
        # laser_bringup_launch,
        driver_node,
        base_node,
        ekf_node,
    ])