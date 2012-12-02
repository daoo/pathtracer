project(pathtracer)

file(GLOB_RECURSE tracer_src *.cpp)
include_directories(. ${FREEIMAGE_INCLUDE_PATH})
add_library(libpathtracer ${tracer_src})
target_link_libraries(libpathtracer ${FREEIMAGE_LIBRARIES})
