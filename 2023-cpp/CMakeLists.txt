cmake_minimum_required(VERSION 3.20)
project(2023_cpp)

set(CMAKE_CXX_STANDARD 26)

if(ASAN)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=address -fsanitize-address-use-after-scope -fno-omit-frame-pointer")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address -fsanitize-address-use-after-scope -fno-omit-frame-pointer")
endif()

add_executable(day8 day-08/main.cpp)
