#include "gui.hpp"

void initGUI(GUI& gui)
{
  glewInit();

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
  glGenVertexArrays(1, &vertexArrayObject);
  glBindVertexArray(vertexArrayObject);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(0);

  // ----------------------------------------------------------------------
  // Create shader
  shaderProgram = loadShaderProgram(VERTEX_SHADER, FRAGMENT_SHADER);
  glBindAttribLocation(shaderProgram, 0, "position");
  glBindFragDataLocation(shaderProgram, 0, "fragmentColor");

  linkShaderProgram(shaderProgram);

  uniformFramebuffer        = glGetUniformLocation(shaderProgram, "framebuffer");
  uniformFramebufferSamples = glGetUniformLocation(shaderProgram, "framebufferSamples");

  // ----------------------------------------------------------------------
  // Create framebuffer texture
  glGenTextures(1, &framebufferTexture);

  // ---------------------------------------------------------------------
  // Set up OpenGL state for use in Display
  glBindVertexArray(vertexArrayObject);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, framebufferTexture);

  glUseProgram(shaderProgram);
  glUniform1i(uniformFramebuffer, 0);
  glUseProgram(0);

  CHECK_GL_ERROR();
}

void trace(GUI& gui)
{
  gui.m_pathtracer->tracePrimaryRays(gui.m_rand, *gui.m_buffer);
}

void render(GUI& gui)
{
  glUseProgram(g_gui.shaderProgram);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      g_gui.m_buffer->width(),
      g_gui.m_buffer->height(),
      0, GL_RGB, GL_FLOAT, g_gui.m_buffer->data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(g_gui.uniformFramebufferSamples, g_gui.m_buffer->samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();
}
