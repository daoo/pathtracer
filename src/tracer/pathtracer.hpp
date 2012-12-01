#ifndef PATHTRACER_HPP_NVGMZUSY
#define PATHTRACER_HPP_NVGMZUSY

#include "math/ray.hpp"
#include "mcsampling.hpp"
#include "scene.hpp"

#include <glm/glm.hpp>

constexpr size_t PT_MAX_BOUNCES = 16;
constexpr float PT_EPSILON      = 0.00001f;

class Pathtracer {
  public:
    Pathtracer(size_t, size_t);
    ~Pathtracer();

    Scene* m_scene;
    size_t m_selectedCamera;

    size_t m_frameBufferWidth, m_frameBufferHeight;
    size_t m_frameBufferSamples;
    std::vector<glm::vec3> m_frameBuffer;

    void restart();
    void resize(int, int);
    void tracePrimaryRays();

    glm::vec3 Li(const math::Ray&, const Intersection&);
    glm::vec3 Lenvironment(const math::Ray&);
};

#endif /* end of include guard: PATHTRACER_HPP_NVGMZUSY */
