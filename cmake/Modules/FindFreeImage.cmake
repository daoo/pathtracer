# Find the FreeImage Library
#
# This module defines the following variables:
#   FREEIMAGE_INCLUDE_DIRS - include directories for FreeImage
#   FREEIMAGE_LIBRARIES    - libraries to link against FreeImage
#   FREEIMAGE_FOUND        - true if FreeImage has been found and can be used

find_path(FREEIMAGE_INCLUDE_DIR FreeImage.h)
find_library(FREEIMAGE_LIBRARY FreeImage NAMES freeimage FreeImage)

set(FREEIMAGE_INCLUDE_DIRS ${FREEIMAGE_INCLUDE_DIR})
set(FREEIMAGE_LIBRARIES ${FREEIMAGE_LIBRARY})

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(FREEIMAGE DEFAULT_MSG
  FREEIMAGE_LIBRARY FREEIMAGE_INCLUDE_DIR)

mark_as_advanced(FREEIMAGE_INCLUDE_DIR FREEIMAGE_LIBRARY)
