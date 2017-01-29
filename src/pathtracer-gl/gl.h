#ifndef PATHTRACER_GL_GL_H_
#define PATHTRACER_GL_GL_H_

#include <GL/glew.h>
#include <GL/freeglut_std.h>
#include <string>

#define CHECK_GL_ERROR() \
  { check_gl_error(__FILE__, __LINE__); }

void check_gl_error(const std::string& file, unsigned int line);

GLuint load_shader_program(const char*, const char*);
void link_shader_program(GLuint);

#endif  // PATHTRACER_GL_GL_H_
