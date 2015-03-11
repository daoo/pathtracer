include_directories(
  .
  ${Boost_INCLUDE_DIR}
  ${FREEIMAGE_INCLUDE_PATH}
  ${GLM_INCLUDE_DIRS}
  )

file(GLOB_RECURSE trace_src *.hpp *.cpp)

add_library(trace ${trace_src})

target_link_libraries(trace
  ${Boost_LIBRARIES}
  ${FREEIMAGE_LIBRARIES}
  )
