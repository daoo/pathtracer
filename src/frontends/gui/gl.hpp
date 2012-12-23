#ifndef GLUTIL_HPP_VK0PY24W
#define GLUTIL_HPP_VK0PY24W

#include <string>
#include <cassert>
#include <GL/glew.h>

#define CHECK_GL_ERROR() { checkGLError(__FILE__, __LINE__); }

void checkGLError(const std::string& file, unsigned int line);

GLuint loadShaderProgram(const std::string&, const std::string&);
void linkShaderProgram(GLuint);

#endif /* end of include guard: GLUTIL_HPP_VK0PY24W */
