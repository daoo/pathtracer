#include "gui.hpp"

#include "gl.hpp"
#include "shaders.hpp"

#include "trace/path.hpp"

#include <algorithm>

using namespace std::experimental::filesystem;
using namespace std;
using namespace trace;
using namespace util;

GUI::GUI(
    const path& dir,
    const path& file,
    const Scene& scene,
    unsigned int subsampling)
  : m_rand()
  , m_screenshot_dir(dir)
  , m_obj_file(file)
  , m_scene(scene)
  , m_camera(0)
  , m_width(500)
  , m_height(500)
  , m_pinhole(scene.cameras[0], 500, 500)
  , m_buffer(500, 500)
  , m_subsampling(subsampling)
{
}

unsigned int GUI::samples() const
{
  return m_buffer.samples();
}

unsigned int GUI::subsampling() const
{
  return m_subsampling;
}

void GUI::increase_subsampling()
{
  m_subsampling += 1;
  restart();
}

void GUI::decrease_subsampling()
{
  m_subsampling = std::max(1U, m_subsampling - 1);
  restart();
}

void GUI::init_gl()
{
  glDisable(GL_CULL_FACE);

  const float positions[] = {
     1.0f, -1.0f, 0.0f,
     1.0f,  1.0f, 0.0f,
    -1.0f, -1.0f, 0.0f,
    -1.0f,  1.0f, 0.0f
  };

  GLuint position_buffer;
  glGenBuffers(1, &position_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);

  // ----------------------------------------------------------------------
  // Connect triangle data with the vertex array object
  glGenVertexArrays(1, &m_vertex_array_object);
  glBindVertexArray(m_vertex_array_object);
  glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(0);

  // ----------------------------------------------------------------------
  // Create shader
  m_shader_program = load_shader_program(VERTEX_SHADER, FRAGMENT_SHADER);
  glBindAttribLocation(m_shader_program, 0, "position");
  glBindFragDataLocation(m_shader_program, 0, "color");

  link_shader_program(m_shader_program);

  m_uniform_framebuffer         = glGetUniformLocation(m_shader_program, "framebuffer");
  m_uniform_framebuffer_samples = glGetUniformLocation(m_shader_program, "samples");

  // ----------------------------------------------------------------------
  // Create framebuffer texture
  glGenTextures(1, &m_framebuffer_texture);

  // ---------------------------------------------------------------------
  // Set up OpenGL state for use in Display
  glBindVertexArray(m_vertex_array_object);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, m_framebuffer_texture);

  glUseProgram(m_shader_program);
  glUniform1i(m_uniform_framebuffer, 0);
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
  pathtrace(
      m_scene.kdtree,
      m_scene.lights,
      m_pinhole,
      m_rand,
      m_buffer);
}

void GUI::render() const
{
  glUseProgram(m_shader_program);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      m_buffer.width(),
      m_buffer.height(),
      0, GL_RGB, GL_FLOAT, m_buffer.data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(m_uniform_framebuffer_samples, m_buffer.samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();
}

void GUI::save_screenshot() const
{
  string name = nice_name(m_obj_file, m_width, m_height, m_buffer.samples());
  write_image(
      next_free_name(m_screenshot_dir, name, ".png").string(),
      m_buffer);
}

void GUI::restart()
{
  unsigned int w = m_width / m_subsampling;
  unsigned int h = m_height / m_subsampling;

  m_pinhole = Pinhole(m_scene.cameras[m_camera], w, h);
  m_buffer  = SampleBuffer(w, h);
}
