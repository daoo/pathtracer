project(cmd)

include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  )

file(GLOB_RECURSE cmd_src *.hpp *.cpp)
add_executable(pathtracer ${cmd_src})
target_link_libraries(pathtracer
  trace
  )
