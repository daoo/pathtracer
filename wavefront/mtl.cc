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
    Parse parse(line.c_str());
    parse.SkipWhitespace();

    if (parse.AtEnd()) continue;

    if (parse.Match("newmtl")) {
      parse.SkipWhitespace();
      mtl.materials.push_back({parse.ParseString(),
                               "",
                               {0.7f, 0.7f, 0.7f},
                               {1.0f, 1.0f, 1.0f},
                               {0.0f, 0.0f, 0.0f},
                               0.001f,
                               0.0f,
                               0.0f,
                               0.0f,
                               1.0f});
    } else if (parse.Match("newlight")) {
      mtl.lights.push_back(
          {{0.0f, 0.0f, 0.0f}, {1.0f, 1.0f, 1.0f}, 0.1f, 10.0f});
    } else if (parse.Match("newcamera")) {
      mtl.cameras.push_back(
          {{7.0f, 5.0f, 6.0f}, {0.0f, 0.0f, 0.0f}, {0.0f, 1.0f, 0.0f}, 10.0f});
    } else if (parse.Match(TOKEN_MTL_DIFFUSE)) {
      parse.SkipWhitespace();
      mtl.materials.back().diffuse = parse.ParseVec3();
    } else if (parse.Match(TOKEN_MTL_DIFFUSE_MAP)) {
      parse.SkipWhitespace();
      mtl.materials.back().diffuse_map = parse.ParseString();
    } else if (parse.Match(TOKEN_MTL_EMITTANCE)) {
      parse.SkipWhitespace();
      mtl.materials.back().emittance = parse.ParseVec3();
    } else if (parse.Match(TOKEN_MTL_IOR)) {
      parse.SkipWhitespace();
      mtl.materials.back().ior = parse.ParseFloat();
    } else if (parse.Match(TOKEN_MTL_REFLECT0)) {
      parse.SkipWhitespace();
      mtl.materials.back().refl0 = parse.ParseFloat();
    } else if (parse.Match(TOKEN_MTL_REFLECT90)) {
      parse.SkipWhitespace();
      mtl.materials.back().refl90 = parse.ParseFloat();
    } else if (parse.Match(TOKEN_MTL_ROUGHNESS)) {
      parse.SkipWhitespace();
      mtl.materials.back().roughness = parse.ParseFloat();
    } else if (parse.Match(TOKEN_MTL_SPECULAR)) {
      parse.SkipWhitespace();
      mtl.materials.back().specular = parse.ParseVec3();
    } else if (parse.Match(TOKEN_MTL_TRANSPARANCY)) {
      parse.SkipWhitespace();
      mtl.materials.back().transparency = parse.ParseFloat();
    } else if (parse.Match(TOKEN_LIGHT_COLOR)) {
      parse.SkipWhitespace();
      mtl.lights.back().color = parse.ParseVec3();
    } else if (parse.Match(TOKEN_LIGHT_INTENSITY)) {
      parse.SkipWhitespace();
      mtl.lights.back().intensity = parse.ParseFloat();
    } else if (parse.Match(TOKEN_LIGHT_POSITION)) {
      parse.SkipWhitespace();
      mtl.lights.back().center = parse.ParseVec3();
    } else if (parse.Match(TOKEN_LIGHT_RADIUS)) {
      parse.SkipWhitespace();
      mtl.lights.back().radius = parse.ParseFloat();
    } else if (parse.Match(TOKEN_CAMERA_FOV)) {
      parse.SkipWhitespace();
      mtl.cameras.back().fov = parse.ParseFloat();
    } else if (parse.Match(TOKEN_CAMERA_POSITION)) {
      parse.SkipWhitespace();
      mtl.cameras.back().position = parse.ParseVec3();
    } else if (parse.Match(TOKEN_CAMERA_TARGET)) {
      parse.SkipWhitespace();
      mtl.cameras.back().target = parse.ParseVec3();
    } else if (parse.Match(TOKEN_CAMERA_UP)) {
      parse.SkipWhitespace();
      mtl.cameras.back().up = parse.ParseVec3();
    }

    ++line_index;
  }

  return mtl;
}
}  // namespace wavefront
