#ifndef OBJMODEL_HPP_BZNJNMXQ
#define OBJMODEL_HPP_BZNJNMXQ

#include <fstream>
#include <glm/glm.hpp>
#include <map>
#include <string>
#include <vector>

class OBJModel
{
  public:
    void load(const std::string& fileName);

    struct Light
    {
      glm::vec3 position;
      glm::vec3 color;
      float radius;
      float intensity;
    };
    std::map<std::string, Light> m_lights;

    struct Camera
    {
      glm::vec3 position;
      glm::vec3 target;
      glm::vec3 up;
      float fov;
    };
    std::map<std::string, Camera> m_cameras;

    struct Material
    {
      glm::vec3 diffuseReflectance;
      std::string diffuseReflectanceMap;
      glm::vec3 specularReflectance;
      glm::vec3 emittance;
      float specularRoughness;
      float transparency;
      float reflAt0Deg;
      float reflAt90Deg;
      float indexOfRefraction;
    };
    std::map<std::string, Material> m_materials;

    struct Chunk
    {
      Material* material;

      // Data on host
      std::vector<glm::vec3> positions;
      std::vector<glm::vec3> normals;
      std::vector<glm::vec2> uvs;
    };
    std::vector<Chunk> m_chunks;

  private:
    void loadOBJ(std::ifstream& file, const std::string& basePath);
    void loadMaterials(const std::string& fileName, const std::string& basePath);
};

#endif /* end of include guard: OBJMODEL_HPP_BZNJNMXQ */
