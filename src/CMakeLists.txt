file(GLOB_RECURSE pathtrace_src
  tracer/*.cpp
  math/*.cpp
  util/image.cpp
  util/objmodel.cpp)

include_directories(. ${FREEIMAGE_INCLUDE_PATH})
add_executable(pathtracer cmdline.cpp ${pathtrace_src})
target_link_libraries(pathtracer ${FREEIMAGE_LIBRARIES})

include_directories(
  ${GLEW_INCLUDE_DIRS}
  ${GLUT_INCLUDE_DIR}
  ${OPENGL_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  )

add_executable(pathtracer-gl glut.cpp ${pathtrace_src} util/gl.cpp)
target_link_libraries(pathtracer-gl
  ${FREEIMAGE_LIBRARIES}
  ${OPENGL_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLUT_LIBRARIES}
  )
