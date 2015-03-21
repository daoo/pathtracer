add_subdirectory(cmd)
add_subdirectory(gui)

add_executable(sizes sizes.cpp)
target_link_libraries(sizes trace)

add_executable(camera-tester camera-tester.cpp)
target_link_libraries(camera-tester trace)

add_executable(print-tree print-tree.cpp)
target_link_libraries(print-tree trace)

add_executable(print-obj print-obj.cpp)
target_link_libraries(print-obj trace)
