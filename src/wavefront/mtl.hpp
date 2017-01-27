#ifndef MTL_HPP_XZYN6ESW
#define MTL_HPP_XZYN6ESW

#include <experimental/filesystem>
#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace {
namespace wavefront {
struct Material {
  std::string name;
  std::string diffuse_map;
  glm::vec3 diffuse;
  glm::vec3 specular;
  glm::vec3 emittance;
  float roughness;
  float transparency;
  float refl0;
  float refl90;
  float ior;
};

struct Light {
  glm::vec3 center;
  glm::vec3 color;
  float radius;
  float intensity;
};

struct Camera {
  glm::vec3 position;
  glm::vec3 target;
  glm::vec3 up;
  float fov;
};

struct Mtl {
  std::vector<Material> materials;
  std::vector<Light> lights;
  std::vector<Camera> cameras;
};

Mtl load_mtl(const std::experimental::filesystem::path&);
}
}

#endif /* end of include guard: MTL_HPP_XZYN6ESW */
