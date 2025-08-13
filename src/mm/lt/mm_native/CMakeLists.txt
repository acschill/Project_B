cmake_minimum_required(VERSION 3.15)
project(mm_native LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(mm_native
    src/fast_ring_buffer.cpp
    src/crc32c.cpp
    src/event_codec.cpp
    src/time_monotonic.cpp
    src/prio_queue.cpp
    src/watchdog.cpp
    src/metrics_exporter.cpp
)
target_include_directories(mm_native PUBLIC include)