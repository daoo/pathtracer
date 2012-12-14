cmake_minimum_required(VERSION 2.8)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")

project(pathtracer)

find_package(Boost REQUIRED COMPONENTS system filesystem mpi)
find_package(FreeImage REQUIRED)
find_package(GLM REQUIRED)

find_package(GLEW)
find_package(GLUT)
find_package(MPI)
find_package(OpenGL)

set(build_mpi_frontend ${MPI_CXX_FOUND})
set(build_gui_frontend ${OPENGL_FOUND})

message(STATUS "MPI frontend: ${build_mpi_frontend}")
message(STATUS "OpenGL frontend: ${build_gui_frontend}")

add_definitions(-Wall -Wextra -std=c++11)
add_subdirectory(src)
add_subdirectory(tests)
