#include <GL/glew.h>
#include <GL/glut.h>

#include <boost/filesystem/convenience.hpp>
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

using namespace boost::filesystem;
using namespace std;
using namespace util;

GUI* g_gui;

void display()
{
  Clock clock;
  clock.start();
  g_gui->trace();
  clock.stop();

  g_gui->render();
  glutSwapBuffers();

  // Print some useful information
  cout << "Seconds per frame: " << clock.length<float, ratio<1>>() << "\n"
       << "Samples per pixel: " << g_gui->samples() << "\n"
       << "Subsampling: " << g_gui->subsampling() << "\n"
       << "\n";
}

void reshape(int w, int h)
{
  g_gui->resize(w, h);
}

void idle()
{
  glutPostRedisplay();
}

void handleKeys(unsigned char key, int, int)
{
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 's') {
    g_gui->increaseSubsampling();
  } else if (key == 'S') {
    g_gui->decreaseSubsampling();
  } else if (key == 'p') {
    g_gui->saveScreenshot();
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
      Scene scene(model);

#ifdef NDEBUG
      constexpr size_t SUBSAMPLING = 1;
#else
      constexpr size_t SUBSAMPLING = 4;
#endif

      string name = basename(change_extension(obj_file, ""));

      cout << name << "\n";
      g_gui = new GUI(screenshot_dir, name, scene, SUBSAMPLING);

      g_gui->initGL();
      g_gui->resize(512, 512);

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
