project(cmd)

include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  )

file(GLOB_RECURSE cmd_src *.hpp *.cpp)
add_executable(schwar ${cmd_src})
target_link_libraries(schwar
  util
  pathtracer
  ${Boost_LIBRARIES}
  ${CMAKE_THREAD_LIBS_INIT}
  )
