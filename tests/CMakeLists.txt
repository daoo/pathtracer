project(tests)

include_directories(. ../src/)

add_executable(sizes sizes.cpp)
target_link_libraries(sizes util pathtracer)

add_executable(print-tree print-tree.cpp)
target_link_libraries(print-tree util pathtracer)

add_executable(print-obj print-obj.cpp)
target_link_libraries(print-obj util pathtracer)
