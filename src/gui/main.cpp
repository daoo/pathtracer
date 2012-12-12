#include <GL/glew.h>
#include <GL/glut.h>

#include <iostream>
#include <sstream>

#include "gl.hpp"
#include "pathtracer/pathtracer.hpp"
#include "shaders.hpp"
#include "util/clock.hpp"
#include "util/samplebuffer.hpp"
#include "util/strings.hpp"

using namespace std;
using namespace util;

FastRand g_rand;
Pathtracer* g_pathtracer;
SampleBuffer* g_buffer;
Scene* g_scene;
string g_img_file;

size_t width, height;

GLuint framebufferTexture;
GLuint shaderProgram;
GLuint uniformFramebuffer, uniformFramebufferSamples;
GLuint vertexArrayObject;

#ifndef NDEBUG
int g_subsample = 4;
#else
int g_subsample = 1;
#endif

void initGL() {
  glewInit();
  startupGLDiagnostics();

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

void display() {
  Clock clock;
  clock.start();
  g_pathtracer->tracePrimaryRays(g_rand, *g_buffer);
  clock.stop();

  glUseProgram(shaderProgram);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      g_buffer->width(),
      g_buffer->height(),
      0, GL_RGB, GL_FLOAT, g_buffer->data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(uniformFramebufferSamples, g_buffer->samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();

  // Print some useful information
  cout << "Seconds per frame: " << clock.length<float, ratio<1>>() << "\n"
       << "Samples per pixel: " << g_buffer->samples() << "\n"
       << "Subsampling: " << g_subsample << "\n"
       << "\n";

  glutSwapBuffers();
}

void restart(size_t w, size_t h, size_t camera) {
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, w, h);

  size_t nw = w / g_subsample;
  size_t nh = h / g_subsample;

  delete g_pathtracer;
  g_pathtracer = new Pathtracer(*g_scene, camera, nw, nh);
  delete g_buffer;
  g_buffer = new SampleBuffer(nw, nh);
}

void reshape(int w, int h) {
  width  = w;
  height = h;

  restart(width, height, 0);
}

void idle() {
  glutPostRedisplay();
}

void handleKeys(unsigned char key, int, int) {
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 'c') {
    restart(width, height, 0);
  } else if (key == 's') {
    g_subsample += 1;
    restart(width, height, 0);
  } else if (key == 'S') {
    g_subsample = max(1, g_subsample - 1);
    restart(width, height, 0);
  } else if (key == 'p') {
    writeImage(g_img_file, *g_buffer);
  }
}

int main(int argc, char *argv[]) {
  if (argc >= 2) {
    try {
      glutInit(&argc, argv);
      glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
      glutInitWindowSize(512, 512);
      glutCreateWindow("Simple Pathtracer");
      glutKeyboardFunc(handleKeys);
      glutReshapeFunc(reshape);
      glutIdleFunc(idle);
      glutDisplayFunc(display);

      string obj_file = argv[1];
      g_img_file = argv[2];

      initGL();

      OBJModel model;
      model.load(obj_file);
      g_scene = new Scene(model);

      restart(512, 512, 0);

      glEnable(GL_FRAMEBUFFER_SRGB);
      glutMainLoop();  /* start the program main loop */
    } catch (const string& err) {
      cerr << "Caught error in main():\n" << err;
    }
  } else {
    cerr << "Usage: pathtracer-gl model.obj output.png\n";
  }

  return 0;
}
