cmake_minimum_required(VERSION 2.8)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")

project(pathtracer)

find_package(Boost REQUIRED COMPONENTS system filesystem program_options)
find_package(FreeImage REQUIRED)
find_package(GLM REQUIRED)

find_package(GLEW)
find_package(GLUT)
find_package(OpenGL)

if (OPENGL_FOUND AND GLEW_FOUND AND GLUT_FOUND)
  set(build_gui_frontend TRUE)
else ()
  set(build_gui_frontend FALSE)
endif ()

message(STATUS "OpenGL frontend: ${build_gui_frontend}")

add_definitions(-std=c++11)
add_subdirectory(src)
