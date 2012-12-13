#include <GL/glew.h>
#include <GL/glut.h>

#include <glm/glm.hpp>
#include <iostream>
#include <sstream>

#include "gui.hpp"

#include "pathtracer/pathtracer.hpp"
#include "util/clock.hpp"
#include "util/path.hpp"
#include "util/path.hpp"
#include "util/samplebuffer.hpp"
#include "util/strings.hpp"

using namespace std;
using namespace util;

GUI* g_gui;

void display()
{
  Clock clock;
  clock.start();
  trace(*g_gui);
  clock.stop();

  render(*g_gui);
  glutSwapBuffers();

  // Print some useful information
  cout << "Seconds per frame: " << clock.length<float, ratio<1>>() << "\n"
       << "Samples per pixel: " << g_gui->m_buffer->samples() << "\n"
       << "Subsampling: " << g_gui->m_subsample << "\n"
       << "\n";
}

void reshape(int w, int h)
{
  g_gui->m_width  = w;
  g_gui->m_height = h;

  restart(*g_gui, w, h);
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
    restart(*g_gui, g_gui->m_width, g_gui->m_height);
  } else if (key == 's') {
    g_gui->m_subsample += 1;
    restart(*g_gui, g_gui->m_width, g_gui->m_height);
  } else if (key == 'S') {
    g_gui->m_subsample = glm::max(1UL, g_gui->m_subsample - 1);
    restart(g_gui->m_width, g_gui->m_height, 0);
  } else if (key == 'p') {
    stringstream name;
    name << g_gui.m_scene_name << "_"
         << g_gui.m_width << "x" << g_gui.m_height << "_"
         << g_gui.m_buffer->samples();

    writeImage(
        nextFreeName(g_gui.m_screenshot_dir, name.str(), ".png"),
        *g_gui.m_buffer);
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
      glewInit();

      string obj_file       = argv[1];
      string screenshot_dir = argv[2];

      OBJModel model;
      model.load(obj_file);
      Scene scene = Scene(model);

      initGUI(g_gui, obj_file, screenshot_dir, scene);

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
