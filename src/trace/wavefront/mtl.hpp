#ifndef MTL_HPP_XZYN6ESW
#define MTL_HPP_XZYN6ESW

#include <boost/filesystem.hpp>
#include <glm/glm.hpp>
#include <string>
#include <vector>

namespace trace
{
  namespace wavefront
  {
    struct Material
    {
      std::string name;
      std::string diffuseMap;
      glm::vec3 diffuse, specular, emittance;
      float roughness;
      float transparency;
      float reflAt0Deg, reflAt90Deg;
      float ior;
    };

    struct Light
    {
      glm::vec3 center, color;
      float radius, intensity;
    };

    struct Camera
    {
      glm::vec3 position, target, up;
      float fov;
    };

    struct Mtl
    {
      std::vector<Material> materials;
      std::vector<Light> lights;
      std::vector<Camera> cameras;
    };

    Mtl loadMtl(const boost::filesystem::path&);
  }
}

#endif /* end of include guard: MTL_HPP_XZYN6ESW */
