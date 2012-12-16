if (${build_mpi_frontend})
  project(mpi)

  find_package(Boost REQUIRED COMPONENTS system serialization filesystem mpi)

  set(CMAKE_CXX_COMPILE_FLAGS ${CMAKE_CXX_COMPILE_FLAGS} ${MPI_CXX_COMPILE_FLAGS})
  set(CMAKE_CXX_LINK_FLAGS ${CMAKE_CXX_LINK_FLAGS} ${MPI_CXX_LINK_FLAGS})

  include_directories(
    ${Boost_INCLUDE_DIR}
    ${GLM_INCLUDE_DIRS}
    ${MPI_CXX_INCLUDE_DIR}
    )

  file(GLOB_RECURSE mpi_src *.hpp *.cpp)
  add_executable(schwar-mpi ${mpi_src})
  target_link_libraries(schwar-mpi
    util
    pathtracer
    ${Boost_LIBRARIES}
    ${MPI_CXX_LIBRARIES}
    )
endif ()
