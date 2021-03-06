#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include <experimental/filesystem>

#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <iostream>
#include <ratio>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "pathtracer-gl/shaders.h"
#include "trace/camera.h"
#include "trace/pathtracer.h"
#include "trace/samplebuffer.h"
#include "trace/scene.h"
#include "util/clock.h"
#include "util/nicetime.h"
#include "util/path.h"
#include "wavefront/mtl.h"
#include "wavefront/obj.h"

using std::experimental::filesystem::path;

constexpr int OK = 0;
constexpr int ERROR_PARAMS = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM = 3;

class State {
 public:
  State(path out_dir,
        path obj,
        path mtl,
        unsigned int width,
        unsigned int height)
      : max_bounces_(16),
        pathtracer_(max_bounces_),
        out_dir_(out_dir),
        obj_name_(obj.stem()),
        scene_(wavefront::LoadObj(obj), wavefront::LoadMtl(mtl)),
        window_width_(width),
        window_height_(height),
        buffer_(width / subsampling_, height / subsampling_),
        pinhole_(scene_.GetCameras()[camera_], buffer_.aspect_ratio()) {
    glEnable(GL_FRAMEBUFFER_SRGB);
    glDisable(GL_CULL_FACE);

    const float positions[] = {1.0f,  -1.0f, 0.0f, 1.0f,  1.0f, 0.0f,
                               -1.0f, -1.0f, 0.0f, -1.0f, 1.0f, 0.0f};

    GLuint position_buffer;
    glGenBuffers(1, &position_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);

    // ----------------------------------------------------------------------
    // Connect triangle data with the vertex array object
    GLuint vertex_array_object;
    glGenVertexArrays(1, &vertex_array_object);
    glBindVertexArray(vertex_array_object);
    glBindBuffer(GL_ARRAY_BUFFER, position_buffer);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, nullptr);
    glEnableVertexAttribArray(0);

    // ----------------------------------------------------------------------
    // Create shader
    GLuint vshader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vshader, 1, &VERTEX_SHADER, nullptr);
    glCompileShader(vshader);
    GLuint fshader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fshader, 1, &FRAGMENT_SHADER, nullptr);
    glCompileShader(fshader);
    GLuint shader_program = glCreateProgram();
    glAttachShader(shader_program, fshader);
    glAttachShader(shader_program, vshader);
    glBindAttribLocation(shader_program, 0, "position");
    glBindFragDataLocation(shader_program, 0, "color");
    glLinkProgram(shader_program);
    GLint uniform_framebuffer =
        glGetUniformLocation(shader_program, "framebuffer");
    uniform_framebuffer_samples_ =
        glGetUniformLocation(shader_program, "samples");

    // ----------------------------------------------------------------------
    // Create framebuffer texture
    GLuint framebuffer_texture;
    glGenTextures(1, &framebuffer_texture);

    // ---------------------------------------------------------------------
    // Set up OpenGL state for use in Display
    glBindVertexArray(vertex_array_object);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, framebuffer_texture);
    glUseProgram(shader_program);
    glUniform1i(uniform_framebuffer, 0);
  }

  void Render() const {
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB32F,
                 static_cast<GLint>(buffer_.width()),
                 static_cast<GLint>(buffer_.height()), 0, GL_RGB, GL_FLOAT,
                 buffer_.data());
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glUniform1i(uniform_framebuffer_samples_, buffer_.samples());
    glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
  }

  void Update() {
    util::Clock clock;
    pathtracer_.Render(scene_, pinhole_, &buffer_);
    double trace_time = clock.measure<double, std::ratio<1>>();
    std::cout << "\r" << buffer_.samples() << ": " << std::fixed
              << std::setprecision(1) << util::TimeAutoUnit(trace_time)
              << std::flush;
  }

  void UpdateWindowSize(unsigned int width, unsigned int height) {
    if (width != window_width_ || height != window_height_) {
      window_width_ = width;
      window_height_ = height;
      Reset();
    }
  }

  void NextCamera() {
    camera_ =
        (camera_ + 1) % static_cast<unsigned int>(scene_.GetCameras().size());
    Reset();
  }

  void PreviousCamera() {
    camera_ =
        (camera_ - 1) % static_cast<unsigned int>(scene_.GetCameras().size());
    Reset();
  }

  void IncreaseSubsampling() {
    subsampling_ += 1;
    Reset();
  }

  void DecreaseSubsampling() {
    subsampling_ = std::max(1U, subsampling_ - 1);
    Reset();
  }

  void IncreaseBounces() {
    max_bounces_ += 1;
    Reset();
  }

  void DecreaseBounces() {
    max_bounces_ = std::max(1LU, max_bounces_ - 1);
    Reset();
  }

  void SaveScreenshot() const {
    std::string name = util::nice_name(obj_name_, window_width_, window_height_,
                                       buffer_.samples());
    write_image(util::next_free_name(out_dir_, name, ".png").string(), buffer_);
  }

 private:
  size_t max_bounces_ = 16;

  trace::Pathtracer pathtracer_;

  path out_dir_;
  path obj_name_;

#ifdef NDEBUG
  unsigned int subsampling_ = 1;
#else
  unsigned int subsampling_ = 4;
#endif

  trace::Scene scene_;
  unsigned int camera_ = 0;

  unsigned int window_width_, window_height_;

  trace::SampleBuffer buffer_;
  trace::Pinhole pinhole_;

  GLint uniform_framebuffer_samples_;

  void Reset() {
    std::cout << '\n';
    std::cout << "window=" << window_width_ << 'x' << window_height_ << ' ';
    std::cout << "subsampling=" << subsampling_ << ' ';
    std::cout << "camera=" << camera_ << ' ';
    std::cout << "bounces=" << max_bounces_ << '\n';
    buffer_ = trace::SampleBuffer(window_width_ / subsampling_,
                                  window_height_ / subsampling_);
    pinhole_ =
        trace::Pinhole(scene_.GetCameras()[camera_], buffer_.aspect_ratio());
    pathtracer_ = trace::Pathtracer(max_bounces_);
  }
};

static void key_callback(GLFWwindow* window, int key, int, int action, int) {
  if (action != GLFW_PRESS) {
    return;
  }

  State* state = static_cast<State*>(glfwGetWindowUserPointer(window));
  if (key == GLFW_KEY_ESCAPE || key == GLFW_KEY_Q) {
    glfwSetWindowShouldClose(window, GLFW_TRUE);
  } else if (key == GLFW_KEY_RIGHT) {
    state->IncreaseSubsampling();
  } else if (key == GLFW_KEY_LEFT) {
    state->DecreaseSubsampling();
  } else if (key == GLFW_KEY_UP) {
    state->IncreaseBounces();
  } else if (key == GLFW_KEY_DOWN) {
    state->DecreaseBounces();
  } else if (key == GLFW_KEY_HOME) {
    state->PreviousCamera();
  } else if (key == GLFW_KEY_END) {
    state->NextCamera();
  } else if (key == GLFW_KEY_F12) {
    state->SaveScreenshot();
  }
}

int main(int argc, char* argv[]) {
  if (argc != 4) {
    std::cerr << "Usage: " << argv[0] << " OBJ MTL OUTDIR\n";
    return ERROR_PARAMS;
  }

  const char* obj_file_str = argv[1];
  const char* mtl_file_str = argv[2];
  const char* out_dir_str = argv[3];
  path obj_file(obj_file_str);
  path mtl_file(mtl_file_str);
  path out_dir(out_dir_str);

  if (!exists(obj_file)) {
    std::cerr << "Error: file " << obj_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (!exists(mtl_file)) {
    std::cerr << "Error: file " << mtl_file << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }
  if (!is_directory(out_dir)) {
    std::cerr << "Error: " << out_dir << " is not a directory.\n";
    return ERROR_FILE_NOT_FOUND;
  }

  try {
    if (!glfwInit()) {
      std::cerr << "Error: failed to initialize GLFW.\n";
      return ERROR_PROGRAM;
    }

    int width = 512, height = 512;
    GLFWwindow* window =
        glfwCreateWindow(width, height, "C++ Pathtracer", nullptr, nullptr);
    if (!window) {
      std::cerr << "Error: failed to create GLFW Window.\n";
      glfwTerminate();
      return ERROR_PROGRAM;
    }

    glfwMakeContextCurrent(window);
    glfwSetKeyCallback(window, key_callback);

    glewInit();

    glfwGetFramebufferSize(window, &width, &height);
    State state(out_dir, obj_file, mtl_file, static_cast<unsigned int>(width),
                static_cast<unsigned int>(height));
    glfwSetWindowUserPointer(window, &state);
    while (!glfwWindowShouldClose(window)) {
      glfwGetFramebufferSize(window, &width, &height);
      state.UpdateWindowSize(static_cast<unsigned int>(width),
                             static_cast<unsigned int>(height));
      state.Update();
      glViewport(0, 0, width, height);
      state.Render();
      glfwSwapBuffers(window);
      glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
  } catch (const std::runtime_error& ex) {
    std::cerr << "ERROR: " << ex.what() << '\n';
    return ERROR_PROGRAM;
  } catch (const std::string& str) {
    std::cerr << "ERROR: " << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
