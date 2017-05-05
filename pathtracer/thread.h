#ifndef PATHTRACER_THREAD_H_
#define PATHTRACER_THREAD_H_

#include <experimental/filesystem>

void program(const std::experimental::filesystem::path&,
             const std::experimental::filesystem::path&,
             const std::experimental::filesystem::path&,
             unsigned int,
             unsigned int,
             unsigned int,
             unsigned int,
             unsigned int);

#endif  // PATHTRACER_THREAD_H_
