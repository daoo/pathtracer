#include "pathtracer-gl/gl.hpp"
#include "pathtracer-gl/shaders.hpp"
#include "trace/camera.hpp"
#include "trace/fastrand.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "trace/scene.hpp"
#include "util/clock.hpp"
#include "util/path.hpp"
#include "wavefront/mtl.hpp"
#include "wavefront/obj.hpp"
#include <GL/freeglut_std.h>
#include <GL/glew.h>
#include <algorithm>
#include <cstdlib>
#include <experimental/filesystem>
#include <iostream>
#include <ratio>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

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

trace::FastRand g_rand;

std::experimental::filesystem::path g_out_dir;
std::experimental::filesystem::path g_obj_name;

unsigned int g_width = DEFAULT_WIDTH;
unsigned int g_height = DEFAULT_HEIGHT;
unsigned int g_subsampling = SUBSAMPLING;

trace::Scene g_scene;
unsigned int g_camera;

trace::Pinhole* g_pinhole = nullptr;
trace::SampleBuffer* g_buffer = nullptr;

GLuint g_framebuffer_texture;
GLuint g_shader_program;
GLint g_uniforg_framebuffer, g_uniforg_framebuffer_samples;
GLuint g_vertex_array_object;

void restart() {
  unsigned int w = g_width / g_subsampling;
  unsigned int h = g_height / g_subsampling;

  delete g_pinhole;
  delete g_buffer;
  g_pinhole = new Pinhole(g_scene.cameras[g_camera], w, h);
  g_buffer = new SampleBuffer(w, h);
}

void increase_subsampling() {
  g_subsampling += 1;
  restart();
}

void decrease_subsampling() {
  g_subsampling = std::max(1U, g_subsampling - 1);
  restart();
}

void resize(GLint width, GLint height) {
  glClearColor(0.2f, 0.2f, 0.8f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glViewport(0, 0, width, height);

  g_width = static_cast<unsigned int>(width);
  g_height = static_cast<unsigned int>(height);

  restart();
}

void render() {
  glUseProgram(g_shader_program);

  // Create and upload raytracer framebuffer as a texture
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
               static_cast<GLint>(g_buffer->width()),
               static_cast<GLint>(g_buffer->height()), 0, GL_RGB, GL_FLOAT,
               g_buffer->data());
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glUniform1i(g_uniforg_framebuffer_samples, g_buffer->samples());

  glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);

  glUseProgram(0);

  CHECK_GL_ERROR();
}

void save_screenshot() {
  string name = nice_name(g_obj_name, g_width, g_height, g_buffer->samples());
  write_image(next_free_name(g_out_dir, name, ".png").string(), *g_buffer);
}

void display() {
  Clock clock;
  pathtrace(g_scene.kdtree, g_scene.lights, *g_pinhole, g_rand, *g_buffer);
  float trace_time = clock.measure<float, ratio<1>>();

  render();
  glutSwapBuffers();

  cout << g_buffer->samples() << ": " << trace_time << "s\n";
}

void idle() {
  glutPostRedisplay();
}

void handle_keys(unsigned char key, int, int) {
  if (key == 27 || key == 'q') {
    exit(0);
  } else if (key == 's') {
    increase_subsampling();
  } else if (key == 'S') {
    decrease_subsampling();
  } else if (key == 'p') {
    save_screenshot();
  }
}

void init_gl(int* argc, char* argv[]) {
  glutInit(argc, argv);
  glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH);
  glutInitWindowSize(DEFAULT_WIDTH, DEFAULT_HEIGHT);
  glutCreateWindow("Pathtracer GL");
  glutKeyboardFunc(handle_keys);
  glutReshapeFunc(resize);
  glutIdleFunc(idle);
  glutDisplayFunc(display);
  glewInit();

  glDisable(GL_CULL_FACE);

  const float positions[] = {1.0f,  -1.0f, 0.0f, 1.0f,  1.0f, 0.0f,
                             -1.0f, -1.0f, 0.0f, -1.0f, 1.0f, 0.0f};

  GLuint position_buffer;
  glGenBuffers(1, &position_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);

  // ----------------------------------------------------------------------
  // Connect triangle data with the vertex array object
  glGenVertexArrays(1, &g_vertex_array_object);
  glBindVertexArray(g_vertex_array_object);
  glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(0);

  // ----------------------------------------------------------------------
  // Create shader
  g_shader_program = load_shader_program(VERTEX_SHADER, FRAGMENT_SHADER);
  glBindAttribLocation(g_shader_program, 0, "position");
  glBindFragDataLocation(g_shader_program, 0, "color");

  link_shader_program(g_shader_program);

  g_uniforg_framebuffer = glGetUniformLocation(g_shader_program, "framebuffer");
  g_uniforg_framebuffer_samples =
      glGetUniformLocation(g_shader_program, "samples");

  // ----------------------------------------------------------------------
  // Create framebuffer texture
  glGenTextures(1, &g_framebuffer_texture);

  // ---------------------------------------------------------------------
  // Set up OpenGL state for use in Display
  glBindVertexArray(g_vertex_array_object);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, g_framebuffer_texture);

  glUseProgram(g_shader_program);
  glUniform1i(g_uniforg_framebuffer, 0);
  glUseProgram(0);

  CHECK_GL_ERROR();
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
    g_out_dir = out_dir;
    g_obj_name = obj_file.stem();

    g_scene = new_scene(wavefront::load_obj(obj_file), wavefront::load_mtl(mtl_file));

    init_gl(&argc, argv);
    resize(DEFAULT_WIDTH, DEFAULT_HEIGHT);

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
