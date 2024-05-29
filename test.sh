#!/usr/bin/env bash

## Assume ROS2 rolling is installed
source /opt/ros/rolling/setup.bash

## Debug
cargo build || {
  exit
}
# export RUST_BACKTRACE=full
# export RUST_LOG="z=trace"

## Ping & Pong executables
## Assume ros2-latency-test is built and placed in /workspace/src
#
# ## Debug mode
# PING="/workspace/src/ros2-latency-test/target/debug/ping"
# PING="/workspace/src/ros2-latency-test/target/debug/ping_blocking"
# PONG="/workspace/src/ros2-latency-test/target/debug/pong"
#
## Release mode
PING="/workspace/src/ros2-latency-test/target/release/ping_blocking"
PONG="/workspace/src/ros2-latency-test/target/release/pong"

# ## rmw_zenoh_cpp
# ## 1. Assume rmw_zenoh_cpp is installed in /workspace/install
# ## 2. zenohd is required
# source /workspace/install/setup.bash
# export RMW_IMPLEMENTATION=rmw_zenoh_cpp
# export ZENOH_RUNTIME='(rx: (handover: app), net: (handover: app), acc: (handover: app), app: (worker_threads: 1), tx: (worker_threads: 1))'
# echo "RMW: $RMW_IMPLEMENTATION"
# parallel --halt now,success=1 --lb <<EOF
# ros2 run rmw_zenoh_cpp rmw_zenohd
# sleep 1; askset -c 0,2 $PONG
# sleep 2; taskset -c 1,3 $PING
# EOF

## rmw_zenoh_rs
# export LD_LIBRARY_PATH=/$(pwd)/target/debug:$LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/$(pwd)/target/release:$LD_LIBRARY_PATH
export RMW_IMPLEMENTATION=rmw_zenoh_rs
export ZENOH_RUNTIME='(rx: (handover: app), net: (handover: app), acc: (handover: app), app: (worker_threads: 1), tx: (worker_threads: 1))'

# ## rmw_fastrtps_cpp
# export RMW_IMPLEMENTATION=rmw_fastrtps_cpp
# export FASTRTPS_DEFAULT_PROFILES_FILE=/workspace/src/ros2-latency-test/config/disable-fastdds-shm.xml

# ## cyclonedds
# export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
# export CYCLONEDDS_URI=file:///workspace/src/ros2-latency-test/config/disable-cyclonedds-shm.xml

echo "RMW: $RMW_IMPLEMENTATION"
parallel --halt now,success=1 --lb <<EOF
taskset -c 0,2 $PONG
sleep 1; taskset -c 1,3 $PING
EOF
