cmake_minimum_required(VERSION 2.8)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/Modules/")

project(pathtracer)

find_package(FreeImage REQUIRED)
find_package(GLEW REQUIRED)
find_package(GLM REQUIRED)
find_package(GLUT REQUIRED)
find_package(OpenGL REQUIRED)

#find_package(OpenMP)
#if(OPENMP_FOUND)
  #set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${OpenMP_C_FLAGS}")
  #set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${OpenMP_CXX_FLAGS}")
  #set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} ${OpenMP_EXE_LINKER_FLAGS}")
#endif()

add_definitions(-Wall -std=c++11)
add_subdirectory(src)
add_subdirectory(tests)
