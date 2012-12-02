project(tracer)

file(GLOB_RECURSE tracer_src *.cpp)
include_directories(. ${FREEIMAGE_INCLUDE_PATH})
add_library(tracer ${tracer_src})
target_link_libraries(tracer ${FREEIMAGE_LIBRARIES})
