#ifndef GL_HPP_VTQYHMUI
#define GL_HPP_VTQYHMUI

#include <string>
#include <GL/glew.h>

#define CHECK_GL_ERROR() { checkGLError(__FILE__, __LINE__); }

void checkGLError(const std::string& file, unsigned int line);

GLuint loadShaderProgram(const std::string&, const std::string&);
void linkShaderProgram(GLuint);

#endif /* end of include guard: GL_HPP_VTQYHMUI */
