#include "gui.hpp"

#include "gl.hpp"
#include "shaders.hpp"

using namespace boost::filesystem;

GUI::GUI(const path& dir, const std::string& name, const Scene& scene)
  : m_rand()
  , m_screenshot_dir(dir)
  , m_scene_name(name)
  , m_scene(scene)
{
}

void initGUI(GUI& gui)
{
  glDisable(GL_CULL_FACE);

  const float positions[] = {
     1.0f, -1.0f, 0.0f,
     1.0f,  1.0f, 0.0f,
    -1.0f, -1.0f, 0.0f,
    -1.0f,  1.0f, 0.0f
  };

  GLuint positionBuffer;
  glGenBuffers(1, &positionBuffer);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);

  // ----------------------------------------------------------------------
  // Connect triangle data with the vertex array object
  glGenVertexArrays(1, &gui.m_vertexArrayObject);
  glBindVertexArray(gui.m_vertexArrayObject);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(0);

  // ----------------------------------------------------------------------
  // Create shader
  gui.m_shaderProgram = loadShaderProgram(VERTEX_SHADER, FRAGMENT_SHADER);
  glBindAttribLocation(gui.m_shaderProgram, 0, "position");
  glBindFragDataLocation(gui.m_shaderProgram, 0, "fragmentColor");

  linkShaderProgram(gui.m_shaderProgram);

  gui.m_uniformFramebuffer        = glGetUniformLocation(gui.m_shaderProgram, "framebuffer");
  gui.m_uniformFramebufferSamples = glGetUniformLocation(gui.m_shaderProgram, "framebufferSamples");

  // ----------------------------------------------------------------------
  // Create framebuffer texture
  glGenTextures(1, &gui.m_framebufferTexture);

  // ---------------------------------------------------------------------
  // Set up OpenGL state for use in Display
  glBindVertexArray(gui.m_vertexArrayObject);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, gui.m_framebufferTexture);

  glUseProgram(gui.m_shaderProgram);
  glUniform1i(gui.m_uniformFramebuffer, 0);
  glUseProgram(0);

  CHECK_GL_ERROR();
}

void restart(GUI& gui, size_t w, size_t h)
{
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, w, h);

  size_t nw = w / g_gui.m_subsample;
  size_t nh = h / g_gui.m_subsample;

  delete g_gui.m_pathtracer;
  g_gui.m_pathtracer = new Pathtracer(*g_gui.m_scene, camera, nw, nh);
  delete g_gui.m_buffer;
  g_gui.m_buffer = new SampleBuffer(nw, nh);
}

void trace(GUI& gui)
{
  gui.m_pathtracer->tracePrimaryRays(gui.m_rand, *gui.m_buffer);
}

void render(GUI& gui)
{
  glUseProgram(gui.m_shaderProgram);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      gui.m_buffer->width(),
      gui.m_buffer->height(),
      0, GL_RGB, GL_FLOAT, gui.m_buffer->data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(gui.m_uniformFramebufferSamples, gui.m_buffer->samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();
}
