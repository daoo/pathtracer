project(cmdline)

file(GLOB_RECURSE cmdline_src *.hpp *.cpp)
include_directories(.)
add_executable(pathtracer ${cmdline_src})
target_link_libraries(pathtracer util libpathtracer)
