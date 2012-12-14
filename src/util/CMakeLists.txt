project(util)

include_directories(
  ${FREEIMAGE_INCLUDE_PATH}
  ${GLM_INCLUDE_DIRS}
  ${Boost_INCLUDE_DIR}
  )

file(GLOB_RECURSE gui_src *.hpp *.cpp)
add_library(util ${gui_src})
target_link_libraries(util
  ${Boost_LIBRARIES}
  ${FREEIMAGE_LIBRARIES}
  )
