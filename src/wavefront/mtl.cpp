#include "mtl.hpp"

#include "wavefront/parse.hpp"

#include <exception>
#include <fstream>
#include <ostream>

using namespace std::experimental::filesystem;
using namespace glm;
using namespace std;

namespace trace {
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
}

Mtl load_mtl(const path& file) {
  std::ifstream stream(file.string());
  if (!stream.good()) {
    string err = "Failed opening file '";
    err += file.string();
    err += "'";
    throw runtime_error(err);
  }

  Mtl mtl;

  string line;
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
      }

      else if (equal("newlight", token)) {
        mtl.lights.push_back(
            {{0.0f, 0.0f, 0.0f}, {1.0f, 1.0f, 1.0f}, 0.1f, 10.0f});
      }

      else if (equal("newcamera", token)) {
        mtl.cameras.push_back({{7.0f, 5.0f, 6.0f},
                               {0.0f, 0.0f, 0.0f},
                               {0.0f, 1.0f, 0.0f},
                               10.0f});
      }

#define TOKEN_VALUE(list, constant, parse, param, error) \
  else if (equal(constant, token)) {                     \
    list.back().param = parse(token + sizeof(constant)); \
  }

      TOKEN_VALUE(mtl.materials, TOKEN_MTL_DIFFUSE, parse_vec3, diffuse,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_DIFFUSE_MAP, parse_string,
                  diffuse_map, "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_EMITTANCE, parse_vec3, emittance,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_IOR, parse_float, ior,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_REFLECT0, parse_float, refl0,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_REFLECT90, parse_float, refl90,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_ROUGHNESS, parse_float, roughness,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_SPECULAR, parse_vec3, specular,
                  "No material created")
      TOKEN_VALUE(mtl.materials, TOKEN_MTL_TRANSPARANCY, parse_float,
                  transparency, "No material created")

      TOKEN_VALUE(mtl.lights, TOKEN_LIGHT_COLOR, parse_vec3, color,
                  "No light created")
      TOKEN_VALUE(mtl.lights, TOKEN_LIGHT_INTENSITY, parse_float, intensity,
                  "No light created")
      TOKEN_VALUE(mtl.lights, TOKEN_LIGHT_POSITION, parse_vec3, center,
                  "No light created")
      TOKEN_VALUE(mtl.lights, TOKEN_LIGHT_RADIUS, parse_float, radius,
                  "No light created")

      TOKEN_VALUE(mtl.cameras, TOKEN_CAMERA_FOV, parse_float, fov,
                  "No camera created")
      TOKEN_VALUE(mtl.cameras, TOKEN_CAMERA_POSITION, parse_vec3, position,
                  "No camera created")
      TOKEN_VALUE(mtl.cameras, TOKEN_CAMERA_TARGET, parse_vec3, target,
                  "No camera created")
      TOKEN_VALUE(mtl.cameras, TOKEN_CAMERA_UP, parse_vec3, up,
                  "No camera created")

#undef TOKEN_VALUE
    }

    ++line_index;
  }

  return mtl;
}
}
}
