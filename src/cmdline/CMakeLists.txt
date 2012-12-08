project(cmdline)

file(GLOB_RECURSE cmdline_src *.hpp *.cpp)
include_directories(.)
add_executable(pathtracer ${cmdline_src})
target_link_libraries(pathtracer ${CMAKE_THREAD_LIBS_INIT} util libpathtracer)
