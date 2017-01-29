#include "pathtracer-gl/gl.h"

#include <cstddef>
#include <sstream>

namespace {
std::string get_shader_info_log(GLuint obj) {
  GLint log_length = 0;
  GLsizei chars_written = 0;
  char* tmp_log;
  std::string log;

  glGetShaderiv(obj, GL_INFO_LOG_LENGTH, &log_length);

  if (log_length > 0) {
    tmp_log = new char[static_cast<size_t>(log_length)];
    glGetShaderInfoLog(obj, log_length, &chars_written, tmp_log);
    log = tmp_log;
    delete[] tmp_log;
  }

  return log;
}

GLuint create_shader(GLuint type, const char* code) {
  GLuint id = glCreateShader(type);
  glShaderSource(id, 1, &code, nullptr);

  glCompileShader(id);

  int compile_ok = 0;
  glGetShaderiv(id, GL_COMPILE_STATUS, &compile_ok);
  if (!compile_ok) {
    throw get_shader_info_log(id);
  }

  return id;
}
}  // namespace

void check_gl_error(const std::string& file, unsigned int line) {
  std::stringstream ss;
  bool was_error = false;
  for (GLenum gl_err = glGetError(); gl_err != GL_NO_ERROR;
       gl_err = glGetError()) {
    was_error = true;
    ss << "GL Error #" << gl_err << " in File " << file << " at line: " << line
       << "\n";
  }

  if (was_error) {
    throw ss.str();
  }
}

GLuint load_shader_program(const char* vertex_shader,
                           const char* fragment_shader) {
  GLuint vshader = create_shader(GL_VERTEX_SHADER, vertex_shader);
  GLuint fshader = create_shader(GL_FRAGMENT_SHADER, fragment_shader);

  GLuint shader_program = glCreateProgram();
  glAttachShader(shader_program, fshader);
  glDeleteShader(fshader);
  glAttachShader(shader_program, vshader);
  glDeleteShader(vshader);

  CHECK_GL_ERROR();

  return shader_program;
}

void link_shader_program(GLuint shader_program) {
  glLinkProgram(shader_program);
  GLint link_ok = 0;
  glGetProgramiv(shader_program, GL_LINK_STATUS, &link_ok);

  if (!link_ok) {
    throw get_shader_info_log(shader_program);
  }
}
