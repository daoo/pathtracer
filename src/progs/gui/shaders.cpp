#include "shaders.hpp"

using namespace std;

const string FRAGMENT_SHADER =
  "#version 130\n"
  "\n"
  "precision highp float;\n"
  "\n"
  "in  vec2 texcoord;\n"
  "out vec4 color;\n"
  "\n"
  "uniform sampler2D framebuffer;\n"
  "uniform int samples;\n"
  "\n"
  "void main() {\n"
  "  color = (1.0 / float(samples)) *\n"
  "    texture2D(framebuffer, texcoord);\n"
  "}\n";

const string VERTEX_SHADER =
  "#version 130\n"
  "\n"
  "in  vec3 position;\n"
  "out vec2 texcoord;\n"
  "\n"
  "void main() {\n"
    "texcoord    = (position.xy + vec2(1.0)) * 0.5;\n"
    "gl_Position = vec4(position, 1);\n"
  "}\n";
