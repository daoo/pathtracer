if (${build_gui_frontend})
  project(gui)

  include_directories(
    ${Boost_INCLUDE_DIR}
    ${GLEW_INCLUDE_DIRS}
    ${GLM_INCLUDE_DIRS}
    ${GLUT_INCLUDE_DIR}
    ${OPENGL_INCLUDE_DIR}
    )

  file(GLOB_RECURSE gui_src *.hpp *.cpp)
  add_executable(pathtracer-gl ${gui_src})
  target_link_libraries(pathtracer-gl
    util
    trace
    ${OPENGL_LIBRARIES}
    ${GLEW_LIBRARIES}
    ${GLUT_LIBRARIES}
    )
endif ()
