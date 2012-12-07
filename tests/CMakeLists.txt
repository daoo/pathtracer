project(tests)

include_directories(. ../src/)

add_executable(sizes sizes.cpp)
target_link_libraries(sizes util libpathtracer)

add_executable(print-tree printtree.cpp)
target_link_libraries(print-tree util libpathtracer)
