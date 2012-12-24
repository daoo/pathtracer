project(math)

file(GLOB_RECURSE math_src *.hpp *.cpp)
add_library(math ${math_src})
