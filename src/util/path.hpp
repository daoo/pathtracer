#ifndef PATH_HPP_KK7VPDVX
#define PATH_HPP_KK7VPDVX

#include <boost/filesystem.hpp>
#include <string>

namespace util
{
  boost::filesystem::path nextFreeName(const boost::filesystem::path&,
      const std::string&, const std::string&);

  std::string niceName(const boost::filesystem::path&,
      unsigned int, unsigned int, unsigned int);
}

#endif /* end of include guard: PATH_HPP_KK7VPDVX */
