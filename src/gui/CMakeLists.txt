project(gui)

find_package(Boost REQUIRED COMPONENTS system filesystem)
find_package(GLEW REQUIRED)
find_package(GLM REQUIRED)
find_package(GLUT REQUIRED)
find_package(OpenGL REQUIRED)

add_definitions(-Wall -Wextra -std=c++11)
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
  libpathtracer
  ${Boost_LIBRARIES}
  ${OPENGL_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLUT_LIBRARIES}
  )
