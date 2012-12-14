project(util)

find_package(Boost REQUIRED COMPONENTS system filesystem)
find_package(FreeImage REQUIRED)
find_package(GLM REQUIRED)

add_definitions(-Wall -Wextra -std=c++11)
include_directories(
  ${FREEIMAGE_INCLUDE_PATH}
  ${Boost_INCLUDE_DIR}
  )

file(GLOB_RECURSE gui_src *.hpp *.cpp)
add_library(util ${gui_src})
target_link_libraries(util
  ${Boost_LIBRARIES}
  ${FREEIMAGE_LIBRARIES}
  )
