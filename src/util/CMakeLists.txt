project(util)

include_directories(${FREEIMAGE_INCLUDE_PATH} ${Boost_INCLUDE_DIR})

file(GLOB_RECURSE gui_src *.hpp *.cpp)
add_library(util ${gui_src})
target_link_libraries(util ${FREEIMAGE_LIBRARIES} ${Boost_LIBRARIES})
