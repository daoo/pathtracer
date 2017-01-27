#ifndef PATH_HPP_KK7VPDVX
#define PATH_HPP_KK7VPDVX

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
}

#endif /* end of include guard: PATH_HPP_KK7VPDVX */
