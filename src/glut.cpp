#include <GL/glew.h>
#include <GL/glut.h>

#include <iostream>
#include <sstream>

#include "tracer/pathtracer.hpp"
#include "util/gl.hpp"
#include "util/image.hpp"

using namespace std;

Pathtracer* g_pathtracer;
Scene g_scene;

GLuint framebufferTexture;
GLuint shaderProgram;
GLuint uniformFramebuffer, uniformFramebufferSamples;
GLuint vertexArrayObject;

#ifndef NDEBUG
int g_subsample = 4;
#else
int g_subsample = 1;
#endif

constexpr size_t MAX_SAMPLES_PER_PIXEL = 2048;

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
  shaderProgram = loadShaderProgram("data/simple.vert", "data/simple.frag");
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

void printString(int x, int y, const string& str) {
  int currentx = x;
  for (size_t i = 0; i < str.size(); ++i) {
    glWindowPos2i(currentx, y);
    glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12, str[i]);
    currentx += glutBitmapWidth(GLUT_BITMAP_HELVETICA_12, str[i]);
  }
}

void display() {
  int t1 = glutGet(GLUT_ELAPSED_TIME);
  if (g_pathtracer->m_frameBufferSamples < MAX_SAMPLES_PER_PIXEL)
    g_pathtracer->tracePrimaryRays();
  int t2 = glutGet(GLUT_ELAPSED_TIME);

  glUseProgram(shaderProgram);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
      g_pathtracer->m_frameBufferWidth,
      g_pathtracer->m_frameBufferHeight,
      0, GL_RGB, GL_FLOAT, g_pathtracer->m_frameBuffer.data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(uniformFramebufferSamples, g_pathtracer->m_frameBufferSamples);

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();

  // ----------------------------------------------------------------------
  // Print some useful information
  {
    stringstream ss;
    ss << "Seconds per frame: " << 0.001 * float(t2 - t1);
    printString(10, 10+14+14, ss.str());
  }
  {
    stringstream ss;
    ss << "Samples per pixel: " << g_pathtracer->m_frameBufferSamples;
    printString(10, 10+14, ss.str());
  }
  {
    stringstream ss;
    ss << "Subsampling: " << 1.0f / float(g_subsample);
    printString(10, 10+0, ss.str());
  }

  glutSwapBuffers();
}

void restart(size_t w, size_t h, size_t camera) {
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, w, h);

  delete g_pathtracer;
  g_pathtracer = new Pathtracer(w / g_subsample, h / g_subsample, g_scene);
  g_pathtracer->m_selectedCamera = camera;
}

void reshape(int w, int h) {
  restart(w, h, g_pathtracer->m_selectedCamera);
}

void idle() {
  glutPostRedisplay();
}

void handleKeys(unsigned char key, int, int) {
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 'c') {
    restart(g_pathtracer->m_frameBufferWidth, g_pathtracer->m_frameBufferHeight,
        g_pathtracer->m_selectedCamera + 1);
  } else if (key == 's') {
    g_subsample += 1;
    restart(g_pathtracer->m_frameBufferWidth, g_pathtracer->m_frameBufferHeight,
        g_pathtracer->m_selectedCamera);
  } else if (key == 'S') {
    g_subsample = max(1, g_subsample - 1);
    restart(g_pathtracer->m_frameBufferWidth, g_pathtracer->m_frameBufferHeight,
        g_pathtracer->m_selectedCamera);
  } else if (key == 'p') {
    writeImage("screenshot.png",
        g_pathtracer->m_frameBufferWidth, g_pathtracer->m_frameBufferHeight,
        g_pathtracer->m_frameBufferSamples,
        g_pathtracer->m_frameBuffer);
  }
}

int main(int argc, char *argv[]) {
  try {
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
    glutInitWindowSize(512, 512);
    glutCreateWindow("Simple Pathtracer");
    glutKeyboardFunc(handleKeys);
    glutReshapeFunc(reshape);
    glutIdleFunc(idle);
    glutDisplayFunc(display);

    initGL();

    OBJModel model;
    model.load("scenes/cornell.obj");
    //model.load("scenes/cornell_textured.obj");
    //model.load("scenes/cornellbottle2.obj");
    g_scene.buildFromObj(&model);

    restart(512, 512, 0);

    glEnable(GL_FRAMEBUFFER_SRGB);
    glutMainLoop();  /* start the program main loop */
  } catch (const string& err) {
    cerr << "Caught error in main():\n" << err;
  }

  return 0;
}
