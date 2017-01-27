#ifndef GL_HPP_VTQYHMUI
#define GL_HPP_VTQYHMUI

#include <GL/glew.h>
#include <string>

#define CHECK_GL_ERROR() \
  { check_gl_error(__FILE__, __LINE__); }

void check_gl_error(const std::string& file, unsigned int line);

GLuint load_shader_program(const std::string&, const std::string&);
void link_shader_program(GLuint);

#endif /* end of include guard: GL_HPP_VTQYHMUI */
