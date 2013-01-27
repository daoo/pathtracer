#include "gui.hpp"

#include "gl.hpp"
#include "shaders.hpp"
#include "util/path.hpp"

#include <algorithm>

using namespace boost::filesystem;
using namespace std;
using namespace trace;
using namespace util;

GUI::GUI(const path& dir, const path& file, const Scene& scene,
    unsigned int subsampling)
  : m_rand()
  , m_screenshot_dir(dir)
  , m_obj_file(file)
  , m_scene(scene)
  , m_camera(0)
  , m_pathtracer(nullptr)
  , m_buffer(nullptr)
  , m_subsampling(subsampling)
{
}

unsigned int GUI::samples() const
{
  return m_buffer->samples();
}

unsigned int GUI::subsampling() const
{
  return m_subsampling;
}

void GUI::increaseSubsampling()
{
  m_subsampling += 1;
  restart();
}

void GUI::decreaseSubsampling()
{
  m_subsampling = std::max(1U, m_subsampling - 1);
  restart();
}

void GUI::initGL()
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
  glGenVertexArrays(1, &m_vertexArrayObject);
  glBindVertexArray(m_vertexArrayObject);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(0);

  // ----------------------------------------------------------------------
  // Create shader
  m_shaderProgram = loadShaderProgram(VERTEX_SHADER, FRAGMENT_SHADER);
  glBindAttribLocation(m_shaderProgram, 0, "position");
  glBindFragDataLocation(m_shaderProgram, 0, "fragmentColor");

  linkShaderProgram(m_shaderProgram);

  m_uniformFramebuffer        = glGetUniformLocation(m_shaderProgram, "framebuffer");
  m_uniformFramebufferSamples = glGetUniformLocation(m_shaderProgram, "framebufferSamples");

  // ----------------------------------------------------------------------
  // Create framebuffer texture
  glGenTextures(1, &m_framebufferTexture);

  // ---------------------------------------------------------------------
  // Set up OpenGL state for use in Display
  glBindVertexArray(m_vertexArrayObject);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, m_framebufferTexture);

  glUseProgram(m_shaderProgram);
  glUniform1i(m_uniformFramebuffer, 0);
  glUseProgram(0);

  CHECK_GL_ERROR();
}

void GUI::resize(unsigned int w, unsigned int h)
{
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, w, h);

  m_width  = w;
  m_height = h;

  restart();
}

void GUI::trace()
{
  m_pathtracer->tracePrimaryRays(m_rand, *m_buffer);
}

void GUI::render() const
{
  glUseProgram(m_shaderProgram);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      m_buffer->width(),
      m_buffer->height(),
      0, GL_RGB, GL_FLOAT, m_buffer->data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(m_uniformFramebufferSamples, m_buffer->samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();
}

void GUI::saveScreenshot() const
{
  string name = niceName(m_obj_file, m_width, m_height, m_buffer->samples());
  writeImage(
      nextFreeName(m_screenshot_dir, name, ".png").string(),
      *m_buffer);
}

void GUI::restart()
{
  unsigned int w = m_width / m_subsampling;
  unsigned int h = m_height / m_subsampling;

  delete m_pathtracer;
  m_pathtracer = new Pathtracer(m_scene, m_camera, w, h);
  delete m_buffer;
  m_buffer = new SampleBuffer(w, h);
}
