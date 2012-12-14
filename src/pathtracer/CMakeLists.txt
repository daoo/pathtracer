project(libpathtracer)

find_package(Boost REQUIRED COMPONENTS system filesystem)
find_package(FreeImage REQUIRED)
find_package(GLM REQUIRED)

add_definitions(-Wall -Wextra -std=c++11)
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
