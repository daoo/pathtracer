project(mpi)

find_package(Boost REQUIRED COMPONENTS mpi system filesystem)
find_package(GLM REQUIRED)
find_package(MPI REQUIRED)
find_package(Threads REQUIRED)

set(CMAKE_CXX_COMPILE_FLAGS ${CMAKE_CXX_COMPILE_FLAGS} ${MPI_CXX_COMPILE_FLAGS})
set(CMAKE_CXX_LINK_FLAGS ${CMAKE_CXX_LINK_FLAGS} ${MPI_CXX_LINK_FLAGS})

add_definitions(-Wall -Wextra -std=c++11)
include_directories(
  ${Boost_INCLUDE_DIR}
  ${GLM_INCLUDE_DIRS}
  ${MPI_CXX_INCLUDE_DIR}
  )

file(GLOB_RECURSE cmdline_src *.hpp *.cpp)
add_executable(pathtracer-mpi ${cmdline_src})
target_link_libraries(pathtracer-mpi
  util
  libpathtracer
  ${Boost_LIBRARIES}
  ${CMAKE_THREAD_LIBS_INIT}
  ${MPI_CXX_LIBRARIES}
  )
