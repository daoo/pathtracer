#ifndef GUI_HPP_MKHN08BI
#define GUI_HPP_MKHN08BI

#include "pathtracer/pathtracer.hpp"
#include "util/fastrand.hpp"
#include "util/samplebuffer.hpp"

#include <string>

struct GUI
{
  util::FastRand m_rand;

  std::string m_screenshot_dir;

  Scene* m_scene;
  std::string m_scene_name;

  Pathtracer* m_pathtracer;
  util::SampleBuffer* m_buffer;

  size_t m_width, m_height;
  size_t m_subsample;

  GLuint m_framebufferTexture;
  GLuint m_shaderProgram;
  GLuint m_uniformFramebuffer, m_uniformFramebufferSamples;
  GLuint m_vertexArrayObject;
};

void initGUI(GUI& gui);
void trace(GUI& gui);
void render(GUI& gui);

#endif /* end of include guard: GUI_HPP_MKHN08BI */
