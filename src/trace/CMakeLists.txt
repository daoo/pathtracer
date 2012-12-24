project(trace)

include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  ${FREEIMAGE_INCLUDE_PATH}
  )

file(GLOB_RECURSE trace_src *.hpp *.cpp)
add_library(trace ${trace_src})
target_link_libraries(trace
  math
  util
  ${Boost_LIBRARIES}
  ${FREEIMAGE_LIBRARIES}
  )
