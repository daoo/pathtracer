#include <GL/glew.h>
#include <GL/glut.h>

#include <glm/glm.hpp>
#include <iostream>
#include <sstream>

#include "gl.hpp"
#include "gui.hpp"
#include "shaders.hpp"

#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/path.hpp"
#include "util/samplebuffer.hpp"
#include "util/strings.hpp"

using namespace std;
using namespace util;

GUI g_gui;

void display()
{
  Clock clock;
  clock.start();
  trace(g_gui);
  clock.stop();

  render(g_gui);
  glutSwapBuffers();

  // Print some useful information
  cout << "Seconds per frame: " << clock.length<float, ratio<1>>() << "\n"
       << "Samples per pixel: " << g_gui.m_buffer->samples() << "\n"
       << "Subsampling: " << g_gui.m_subsample << "\n"
       << "\n";
}

void restart(size_t w, size_t h, size_t camera)
{
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, w, h);

  size_t nw = w / g_gui.m_subsample;
  size_t nh = h / g_gui.m_subsample;

  delete g_gui.m_pathtracer;
  g_gui.m_pathtracer = new Pathtracer(*g_gui.m_scene, camera, nw, nh);
  delete g_gui.m_buffer;
  g_gui.m_buffer = new SampleBuffer(nw, nh);
}

void reshape(int w, int h)
{
  g_gui.m_width  = w;
  g_gui.m_height = h;

  restart(w, h, 0);
}

void idle()
{
  glutPostRedisplay();
}

void handleKeys(unsigned char key, int, int)
{
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 'c') {
    restart(g_gui.m_width, g_gui.m_height, 0);
  } else if (key == 's') {
    g_gui.m_subsample += 1;
    restart(g_gui.m_width, g_gui.m_height, 0);
  } else if (key == 'S') {
    g_gui.m_subsample = glm::max(1UL, g_gui.m_subsample - 1);
    restart(g_gui.m_width, g_gui.m_height, 0);
  } else if (key == 'p') {
    writeImage(
        nextFreeName(g_gui.m_screenshot_dir, g_gui.m_scene_name, "png"),
        g_gui.m_buffer);
  }
}

int main(int argc, char *argv[])
{
  if (argc >= 3) {
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
      g_gui.m_img_gui.m_file = argv[2];

      initGL();

      OBJModel model;
      model.load(obj_file);
      g_gui.m_scene = new Scene(model);

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
