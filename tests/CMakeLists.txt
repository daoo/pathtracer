project(tests)

file(GLOB_RECURSE tests_src *.hpp *.cpp)
include_directories(. ../src/)

add_executable(sizes sizes)
target_link_libraries(sizes util libpathtracer)
