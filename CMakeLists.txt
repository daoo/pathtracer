cmake_minimum_required(VERSION 2.8)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")

project(pathtracer)

find_package(Boost REQUIRED COMPONENTS system filesystem)
find_package(FreeImage REQUIRED)
find_package(GLEW REQUIRED)
find_package(GLM REQUIRED)
find_package(GLUT REQUIRED)
find_package(OpenGL REQUIRED)
find_package(Threads REQUIRED)

add_definitions(-Wall -std=c++11)
add_subdirectory(src)
add_subdirectory(tests)
