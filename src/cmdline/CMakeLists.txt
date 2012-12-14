project(cmdline)

find_package(Boost REQUIRED COMPONENTS system filesystem)
find_package(GLM REQUIRED)
find_package(Threads REQUIRED)

add_definitions(-Wall -Wextra -std=c++11)
include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  )

file(GLOB_RECURSE cmdline_src *.hpp *.cpp)
add_executable(pathtracer ${cmdline_src})
target_link_libraries(pathtracer
  util
  libpathtracer
  ${Boost_LIBRARIES}
  ${CMAKE_THREAD_LIBS_INIT}
  )
