#include "gl.hpp"

#include <sstream>

using namespace std;

namespace {
  string getShaderInfoLog(GLuint obj)
  {
    int logLength = 0;
    int charsWritten  = 0;
    char *tmpLog;
    string log;

    glGetShaderiv(obj, GL_INFO_LOG_LENGTH, &logLength);

    if (logLength > 0) {
      tmpLog = new char[logLength];
      glGetShaderInfoLog(obj, logLength, &charsWritten, tmpLog);
      log = tmpLog;
      delete[] tmpLog;
    }

    return log;
  }

  GLuint createShader(GLuint type, const string& code)
  {
    const GLchar* str = code.c_str();

    GLuint id = glCreateShader(type);
    glShaderSource(id, 1, &str, NULL);

    glCompileShader(id);

    int compileOk = 0;
    glGetShaderiv(id, GL_COMPILE_STATUS, &compileOk);
    if (!compileOk) {
      throw getShaderInfoLog(id);
    }

    return id;
  }
}

void checkGLError(const string& file, size_t line)
{
  stringstream ss;
  bool wasError = false;
  for (GLenum glErr = glGetError(); glErr != GL_NO_ERROR; glErr = glGetError()) {
    wasError = true;
    ss << "GL Error #" << glErr << " in File " << file << " at line: " << line << "\n";
  }

  if (wasError) {
    throw ss.str();
  }
}

GLuint loadShaderProgram(const string& vertexShader, const string& fragmentShader)
{
  GLuint vShader = createShader(GL_VERTEX_SHADER, vertexShader);
  GLuint fShader = createShader(GL_FRAGMENT_SHADER, fragmentShader);

  GLuint shaderProgram = glCreateProgram();
  glAttachShader(shaderProgram, fShader);
  glDeleteShader(fShader);
  glAttachShader(shaderProgram, vShader);
  glDeleteShader(vShader);

  CHECK_GL_ERROR();

  return shaderProgram;
}

void linkShaderProgram(GLuint shaderProgram)
{
  glLinkProgram(shaderProgram);
  GLint linkOk = 0;
  glGetProgramiv(shaderProgram, GL_LINK_STATUS, &linkOk);

  if (!linkOk) {
    throw getShaderInfoLog(shaderProgram);
  }
}
