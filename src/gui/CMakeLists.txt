project(gui)

include_directories(
  .
  ${GLEW_INCLUDE_DIRS}
  ${GLUT_INCLUDE_DIR}
  ${OPENGL_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  )

file(GLOB_RECURSE gui_src *.hpp *.cpp)
add_executable(pathtracer-gl ${gui_src})
target_link_libraries(pathtracer-gl
  util
  libpathtracer
  ${OPENGL_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLUT_LIBRARIES}
  )
