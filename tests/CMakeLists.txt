project(tests)

include_directories(. ../src/)

add_executable(sizes sizes.cpp)
target_link_libraries(sizes util trace)

add_executable(print-tree print-tree.cpp)
target_link_libraries(print-tree util trace)

add_executable(print-obj print-obj.cpp)
target_link_libraries(print-obj util trace)
