project(cmdline)

file(GLOB_RECURSE cmdline_src *.cpp)
include_directories(. ${TRACER_INCLUDE_DIR})
add_executable(pathtracer ${cmdline_src})
target_link_libraries(pathtracer tracer)
