#include <GL/glew.h>
#include <GL/glut.h>

#include "gui.hpp"

#include "trace/clock.hpp"
#include "trace/path.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "trace/strings.hpp"

#include <boost/filesystem/convenience.hpp>
#include <boost/filesystem/path.hpp>
#include <boost/program_options.hpp>
#include <iostream>

namespace fs = boost::filesystem;
namespace po = boost::program_options;

using namespace std;
using namespace trace;
using namespace util;

constexpr unsigned int DEFAULT_WIDTH  = 512;
constexpr unsigned int DEFAULT_HEIGHT = 512;

#ifdef NDEBUG
constexpr unsigned int SUBSAMPLING = 1;
#else
constexpr unsigned int SUBSAMPLING = 4;
#endif

constexpr int OK                   = 0;
constexpr int ERROR_PARAMS         = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM        = 3;

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
  fs::path outdir, model;

  po::options_description desc("Pathtracer GUI options");
  desc.add_options()
    ("help,h"   , "produce help message")
    ("model,m"  , po::value<fs::path>(&model)  , "obj model")
    ("outdir,o" , po::value<fs::path>(&outdir) , "output directory for screenshots")
    ;

  try {
    po::positional_options_description pd;
    pd.add("model", -1);

    po::variables_map vm;
    po::store(po::command_line_parser(argc, argv).options(desc).positional(pd).run(), vm);
    po::notify(vm);

    if (vm.count("help")) {
      cout << desc << '\n';
      return OK;
    }

    if (!exists(model)) {
      cerr << "ERROR: file " << model << " does not exist.\n";
      return ERROR_FILE_NOT_FOUND;
    }

    const wavefront::Obj obj = wavefront::loadObj(model);
    const wavefront::Mtl mtl = wavefront::loadMtl(model.parent_path() / obj.mtl_lib);
    const Scene scene(obj, mtl);

    string name = basename(change_extension(model, ""));

    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
    glutInitWindowSize(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    glutCreateWindow("Pathtracer GUI");
    glutKeyboardFunc(handleKeys);
    glutReshapeFunc(reshape);
    glutIdleFunc(idle);
    glutDisplayFunc(display);
    glewInit();

    g_gui = new GUI(outdir, name, scene, SUBSAMPLING);
    g_gui->initGL();
    g_gui->resize(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    glEnable(GL_FRAMEBUFFER_SRGB);
    glutMainLoop();  /* start the program main loop */
  } catch (const po::error& ex) {
    cerr << "ERROR: " << ex.what() << "\n\n";
    cout << desc;
    return ERROR_PARAMS;
  } catch (const runtime_error& ex) {
    cerr << "ERROR: " << ex.what() << '\n';
    return ERROR_PROGRAM;
  } catch (const string& str) {
    cerr << "ERROR: " << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
