project(tests)

add_definitions(-Wall -Wextra -std=c++11)
include_directories(. ../src/)

add_executable(sizes sizes.cpp)
target_link_libraries(sizes util libpathtracer)

add_executable(print-tree printtree.cpp)
target_link_libraries(print-tree util libpathtracer)
