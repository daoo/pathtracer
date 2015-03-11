#ifndef OBJ_HPP_BMTCK0HO
#define OBJ_HPP_BMTCK0HO

#include <boost/filesystem.hpp>
#include <glm/glm.hpp>
#include <vector>

namespace trace
{
  namespace wavefront
  {
    struct Point { int v, t, n; };
    struct Face { Point p1, p2, p3; };

    struct Chunk
    {
      Chunk(const std::string& mtl) : material(mtl) { }

      std::vector<Face> polygons;
      std::string material;
    };

    struct Obj
    {
      boost::filesystem::path mtl_lib;

      std::vector<glm::vec3> vertices;
      std::vector<glm::vec3> normals;
      std::vector<glm::vec2> texcoords;
      std::vector<Chunk> chunks;
    };

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

    Obj loadObj(const boost::filesystem::path&);
    Mtl loadMtl(const boost::filesystem::path&);
  }
}

#endif /* end of include guard: OBJ_HPP_BMTCK0HO */
