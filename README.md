# ROS 2 RMW for Eclipse Cyclone Zenoh in Rust

This is a Rust implementation of RMW for [Zenoh](https://github.com/eclipse-zenoh/zenoh). It's still under development. Please use it with caution.

## Why

- Embrace Rust's convenient building system, Cargo. No colcon is needed anymore.
- Pure Rust for safety and efficiency. No overhead between zenoh-c / zenoh-rust translation.
- Leverage the full power of Zenoh Rust library.


## How to use

### Prequsites

- [Install ROS2 rolling](https://docs.ros.org/en/rolling/Installation.html)
- [Rust toolchain](https://www.rust-lang.org/tools/install)

### Build

- Source the ROS2 environment
  ```bash
  source /opt/ros/rolling/setup.bash
  ```
- Build in release mode
  ```bash
  cargo build --release
  ```
  or in debug mode
  ```bash
  cargo build
  ```

### Usage
- Source the library
  ```bash
  export LD_LIBRARY_PATH=/$(pwd)/target/release:$LD_LIBRARY_PATH
  export RMW_IMPLEMENTATION=rmw_zenoh_rs
  ```
- Run the ROS2 application


## Comparison with other RMWs

### Install RMWs

- [rmw_fastrtps_cpp](https://github.com/ros2/rmw_fastrtps) has been installed on the rolling version by default.
- [rmw_cyclonedds_cpp](https://github.com/ros2/rmw_cyclonedds) can be installed via

  ```bash
  sudo apt install ros-rolling-rmw-cyclonedds-cpp
  ```
- [rmw_zenoh_cpp](https://github.com/ros2/rmw_zenoh), please follow the doc to build it.

### Latency Test

We use [ros2-latency-test](https://github.com/YuanYuYuan/ros2-latency-test) to benchmark the latency.

- Build the test
  ```bash
  git clone https://github.com/YuanYuYuan/ros2-latency-test
  source /opt/ros/rolling/setup.bash
  cargo build --release
  ```

- Run the Test

  Open and modify the testing script _./test.sh_ to
  1. specify the path of _ros2-latency-test_ binaryies
  2. specify the path of _rmw_zenoh_cpp_ if needed
  3. select which RMW to test

  ```bash
  ./test.sh
  ```
