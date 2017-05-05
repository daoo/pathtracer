#include "wavefront/mtl.h"

#include <exception>
#include <fstream>
#include <ostream>

#include "wavefront/parse.h"

using std::experimental::filesystem::path;

namespace wavefront {
namespace {
// Mtl tokens
const char TOKEN_MTL_DIFFUSE[] = "kd";
const char TOKEN_MTL_DIFFUSE_MAP[] = "map_kd";
const char TOKEN_MTL_EMITTANCE[] = "emittance";
const char TOKEN_MTL_IOR[] = "indexofrefraction";
const char TOKEN_MTL_REFLECT0[] = "reflat0deg";
const char TOKEN_MTL_REFLECT90[] = "reflat90deg";
const char TOKEN_MTL_ROUGHNESS[] = "specularroughness";
const char TOKEN_MTL_SPECULAR[] = "ks";
const char TOKEN_MTL_TRANSPARANCY[] = "transparency";

// Light tokens
const char TOKEN_LIGHT_COLOR[] = "lightcolor";
const char TOKEN_LIGHT_INTENSITY[] = "lightintensity";
const char TOKEN_LIGHT_POSITION[] = "lightposition";
const char TOKEN_LIGHT_RADIUS[] = "lightradius";

// Camera tokens
const char TOKEN_CAMERA_FOV[] = "camerafov";
const char TOKEN_CAMERA_POSITION[] = "cameraposition";
const char TOKEN_CAMERA_TARGET[] = "cameratarget";
const char TOKEN_CAMERA_UP[] = "cameraup";
}  // namespace

Mtl load_mtl(const path& file) {
  std::ifstream stream(file.string());
  if (!stream.good()) {
    std::string err = "Failed opening file '";
    err += file.string();
    err += "'";
    throw std::runtime_error(err);
  }

  Mtl mtl;

  std::string line;
  unsigned int line_index = 0;
  while (getline(stream, line)) {
    if (!line.empty()) {
      const char* token = skip_whitespace(line.c_str());

      if (equal("newmtl", token)) {
        mtl.materials.push_back({parse_string(skip_whitespace(token + 7)),
                                 "",
                                 {0.7f, 0.7f, 0.7f},
                                 {1.0f, 1.0f, 1.0f},
                                 {0.0f, 0.0f, 0.0f},
                                 0.001f,
                                 0.0f,
                                 0.0f,
                                 0.0f,
                                 1.0f});
      } else if (equal("newlight", token)) {
        mtl.lights.push_back(
            {{0.0f, 0.0f, 0.0f}, {1.0f, 1.0f, 1.0f}, 0.1f, 10.0f});
      } else if (equal("newcamera", token)) {
        mtl.cameras.push_back({{7.0f, 5.0f, 6.0f},
                               {0.0f, 0.0f, 0.0f},
                               {0.0f, 1.0f, 0.0f},
                               10.0f});
      } else if (equal(TOKEN_MTL_DIFFUSE, token)) {
        mtl.materials.back().diffuse =
            parse_vec3(token + sizeof(TOKEN_MTL_DIFFUSE));
      } else if (equal(TOKEN_MTL_DIFFUSE_MAP, token)) {
        mtl.materials.back().diffuse_map =
            parse_string(token + sizeof(TOKEN_MTL_DIFFUSE_MAP));
      } else if (equal(TOKEN_MTL_EMITTANCE, token)) {
        mtl.materials.back().emittance =
            parse_vec3(token + sizeof(TOKEN_MTL_EMITTANCE));
      } else if (equal(TOKEN_MTL_IOR, token)) {
        mtl.materials.back().ior = parse_float(token + sizeof(TOKEN_MTL_IOR));
      } else if (equal(TOKEN_MTL_REFLECT0, token)) {
        mtl.materials.back().refl0 =
            parse_float(token + sizeof(TOKEN_MTL_REFLECT0));
      } else if (equal(TOKEN_MTL_REFLECT90, token)) {
        mtl.materials.back().refl90 =
            parse_float(token + sizeof(TOKEN_MTL_REFLECT90));
      } else if (equal(TOKEN_MTL_ROUGHNESS, token)) {
        mtl.materials.back().roughness =
            parse_float(token + sizeof(TOKEN_MTL_ROUGHNESS));
      } else if (equal(TOKEN_MTL_SPECULAR, token)) {
        mtl.materials.back().specular =
            parse_vec3(token + sizeof(TOKEN_MTL_SPECULAR));
      } else if (equal(TOKEN_MTL_TRANSPARANCY, token)) {
        mtl.materials.back().transparency =
            parse_float(token + sizeof(TOKEN_MTL_TRANSPARANCY));
      } else if (equal(TOKEN_LIGHT_COLOR, token)) {
        mtl.lights.back().color = parse_vec3(token + sizeof(TOKEN_LIGHT_COLOR));
      } else if (equal(TOKEN_LIGHT_INTENSITY, token)) {
        mtl.lights.back().intensity =
            parse_float(token + sizeof(TOKEN_LIGHT_INTENSITY));
      } else if (equal(TOKEN_LIGHT_POSITION, token)) {
        mtl.lights.back().center =
            parse_vec3(token + sizeof(TOKEN_LIGHT_POSITION));
      } else if (equal(TOKEN_LIGHT_RADIUS, token)) {
        mtl.lights.back().radius =
            parse_float(token + sizeof(TOKEN_LIGHT_RADIUS));
      } else if (equal(TOKEN_CAMERA_FOV, token)) {
        mtl.cameras.back().fov = parse_float(token + sizeof(TOKEN_CAMERA_FOV));
      } else if (equal(TOKEN_CAMERA_POSITION, token)) {
        mtl.cameras.back().position =
            parse_vec3(token + sizeof(TOKEN_CAMERA_POSITION));
      } else if (equal(TOKEN_CAMERA_TARGET, token)) {
        mtl.cameras.back().target =
            parse_vec3(token + sizeof(TOKEN_CAMERA_TARGET));
      } else if (equal(TOKEN_CAMERA_UP, token)) {
        mtl.cameras.back().up = parse_vec3(token + sizeof(TOKEN_CAMERA_UP));
      }
    }

    ++line_index;
  }

  return mtl;
}
}  // namespace wavefront
