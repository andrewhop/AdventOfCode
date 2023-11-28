cmake_minimum_required(VERSION 3.0)
project(2023)

set(CMAKE_CXX_STANDARD 11)
enable_language(ASM)

file(GLOB day_paths LIST_DIRECTORIES true "day-*")
foreach(day_path ${day_paths})
    if(IS_DIRECTORY ${day_path})
        file(GLOB sources "${day_path}/*")
        get_filename_component(day ${day_path} NAME)
        add_executable(${day} ${sources})
    endif()
endforeach()