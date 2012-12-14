project(libpathtracer)

include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  ${FREEIMAGE_INCLUDE_PATH}
  )

file(GLOB_RECURSE tracer_src *.hpp *.cpp)
add_library(libpathtracer ${tracer_src})
target_link_libraries(libpathtracer
  util
  ${Boost_LIBRARIES}
  ${FREEIMAGE_LIBRARIES}
  )
