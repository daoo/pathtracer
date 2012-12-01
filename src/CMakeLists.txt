file(GLOB_RECURSE pathtrace_src
  tracer/*.cpp
  math/*.cpp
  util/image.cpp
  util/objmodel.cpp)
include_directories(.)

add_executable(pathtracer main.cpp ${pathtrace_src} util/gl.cpp)
target_link_libraries(pathtracer
  freeimage
  ${OPENGL_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLUT_LIBRARIES}
  )
