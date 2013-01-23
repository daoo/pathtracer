# Find the glm matrix/vector library
#
# This module defines the following variables:
#   GLM_INCLUDE_DIRS - where to find glm/glm.hpp
#   GLM_FOUND        - if the library was successfully located

find_path(GLM_INCLUDE_DIR FreeImage.h)

set(GLM_INCLUDE_DIRS ${GLM_INCLUDE_DIR})

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(GLM DEFAULT_MSG GLM_INCLUDE_DIR)

mark_as_advanced(GLM_INCLUDE_DIR)
