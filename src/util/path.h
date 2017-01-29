#ifndef UTIL_PATH_H_
#define UTIL_PATH_H_

#include <experimental/filesystem>
#include <string>

namespace util {
std::experimental::filesystem::path next_free_name(
    const std::experimental::filesystem::path&,
    const std::string&,
    const std::string&);

std::string nice_name(const std::experimental::filesystem::path&,
                      unsigned int,
                      unsigned int,
                      unsigned int);
}  // namespace util

#endif  // UTIL_PATH_H_
