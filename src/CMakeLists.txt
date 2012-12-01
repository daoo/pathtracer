file(GLOB_RECURSE pathtrace_src
  tracer/*.cpp
  math/*.cpp
  util/image.cpp
  util/objmodel.cpp)
include_directories(.)

add_executable(pathtracer cmdline.cpp ${pathtrace_src})
target_link_libraries(pathtracer freeimage)

add_executable(pathtracer-gl glut.cpp ${pathtrace_src} util/gl.cpp)
target_link_libraries(pathtracer-gl
  freeimage
  ${OPENGL_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLUT_LIBRARIES}
  )
