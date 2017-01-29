#include "pathtracer-gl/gui.hpp"
#include "trace/scene.hpp"
#include "util/clock.hpp"
#include "wavefront/mtl.hpp"
#include "wavefront/obj.hpp"
#include <GL/freeglut_std.h>
#include <GL/glew.h>
#include <cstdlib>
#include <experimental/filesystem>
#include <iostream>
#include <ratio>
#include <sstream>
#include <stdexcept>
#include <string>

using namespace std;
using namespace trace;
using namespace util;
using std::experimental::filesystem::path;

constexpr unsigned int DEFAULT_WIDTH = 512;
constexpr unsigned int DEFAULT_HEIGHT = 512;

#ifdef NDEBUG
constexpr unsigned int SUBSAMPLING = 1;
#else
constexpr unsigned int SUBSAMPLING = 4;
#endif

constexpr int OK = 0;
constexpr int ERROR_PARAMS = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM = 3;

static GUI* g_gui;

void display() {
  Clock clock;
  g_gui->trace();
  float trace_time = clock.measure<float, ratio<1>>();

  g_gui->render();
  glutSwapBuffers();

  // Print some useful information
  cout << "Second(s) per frame: " << trace_time << "\n"
       << "Samples per pixel: " << g_gui->samples() << "\n"
       << "Subsampling: " << g_gui->subsampling() << "\n"
       << "\n";
}

void reshape(int w, int h) {
  g_gui->resize(w, h);
}

void idle() {
  glutPostRedisplay();
}

void handle_keys(unsigned char key, int, int) {
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 's') {
    g_gui->increase_subsampling();
  } else if (key == 'S') {
    g_gui->decrease_subsampling();
  } else if (key == 'p') {
    g_gui->save_screenshot();
  }
}

int main(int argc, char* argv[]) {
  if (argc != 4) {
    cerr << "Usage: " << argv[0] << " OBJ MTL OUTDIR\n";
    return ERROR_PARAMS;
  }

  const char* obj_file_str = argv[1];
  const char* mtl_file_str = argv[2];
  const char* out_dir_str = argv[3];
  path obj_file(obj_file_str);
  path mtl_file(mtl_file_str);
  path out_dir(out_dir_str);

  if (!exists(obj_file)) {
    cerr << "Error: file " << obj_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (!exists(mtl_file)) {
    cerr << "Error: file " << mtl_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (!is_directory(out_dir)) {
    cerr << "Error: " << out_dir << " is not a directory.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  try {
    Scene scene =
        new_scene(wavefront::load_obj(obj_file), wavefront::load_mtl(mtl_file));

    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
    glutInitWindowSize(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    glutCreateWindow("Pathtracer GUI");
    glutKeyboardFunc(handle_keys);
    glutReshapeFunc(reshape);
    glutIdleFunc(idle);
    glutDisplayFunc(display);
    glewInit();

    g_gui = new GUI(out_dir, obj_file.stem(), scene, SUBSAMPLING);
    g_gui->init_gl();
    g_gui->resize(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    glEnable(GL_FRAMEBUFFER_SRGB);
    glutMainLoop(); /* start the program main loop */
  } catch (const runtime_error& ex) {
    cerr << "ERROR: " << ex.what() << '\n';
    return ERROR_PROGRAM;
  } catch (const string& str) {
    cerr << "ERROR: " << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
