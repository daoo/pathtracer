#include "gl.hpp"

#include <iostream>
#include <sstream>

using namespace std;

namespace {
  const char* textFileRead(const string& file) {
    char* content = NULL;

    if (!file.empty()) {
      FILE *fp;
      fp = fopen(file.c_str(), "rt");
      if (fp != NULL) {
        fseek(fp, 0, SEEK_END);
        int count = ftell(fp);
        fseek(fp, 0, SEEK_SET);

        if (count > 0) {
          content        = new char[count+1];
          count          = fread(content, sizeof(char), count, fp);
          content[count] = '\0';
        } else {
          stringstream ss;
          ss << "File '" << file << "' is empty";
          throw ss.str();
        }

        fclose(fp);
      } else {
        stringstream ss;
        ss << "Unable to read file '" << file << "'";
        throw ss.str();
      }
    } else {
      throw string("Empty file name");
    }

    return content;
  }

  string getShaderInfoLog(GLuint obj) {
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
}

void checkGLError(const string& file, size_t line) {
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


void startupGLDiagnostics() {
  if (!GLEW_VERSION_3_0) {
    throw string("OpenGL 3.0 not supported.");
  }
}

GLuint loadShaderProgram(const string& vertexShader, const string& fragmentShader) {
  GLuint vShader = glCreateShader(GL_VERTEX_SHADER);
  GLuint fShader = glCreateShader(GL_FRAGMENT_SHADER);

  const char* vs = textFileRead(vertexShader);
  const char* fs = textFileRead(fragmentShader);

  glShaderSource(vShader, 1, &vs, NULL);
  glShaderSource(fShader, 1, &fs, NULL);
  // text data is not needed beyond this point
  delete [] vs;
  delete [] fs;

  glCompileShader(vShader);
  int compileOk = 0;
  glGetShaderiv(vShader, GL_COMPILE_STATUS, &compileOk);
  if (!compileOk) {
    throw getShaderInfoLog(vShader);
  }

  glCompileShader(fShader);
  glGetShaderiv(fShader, GL_COMPILE_STATUS, &compileOk);
  if (!compileOk) {
    throw getShaderInfoLog(fShader);
  }

  GLuint shaderProgram = glCreateProgram();
  glAttachShader(shaderProgram, fShader);
  glDeleteShader(fShader);
  glAttachShader(shaderProgram, vShader);
  glDeleteShader(vShader);
  CHECK_GL_ERROR();

  return shaderProgram;
}

void linkShaderProgram(GLuint shaderProgram) {
  glLinkProgram(shaderProgram);
  GLint linkOk = 0;
  glGetProgramiv(shaderProgram, GL_LINK_STATUS, &linkOk);

  if (!linkOk) {
    throw getShaderInfoLog(shaderProgram);
  }
}
