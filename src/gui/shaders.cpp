#include "shaders.hpp"

using namespace std;

const string FRAGMENT_SHADER =
  "#version 130\n"
  "\n"
  "precision highp float;\n"
  "\n"
  "in  vec2 texCoord;\n"
  "out vec4 fragmentColor;\n"
  "\n"
  "uniform sampler2D framebuffer;\n"
  "uniform int framebufferSamples;\n"
  "\n"
  "void main() {\n"
  "  fragmentColor = (1.0 / float(framebufferSamples)) *\n"
  "    texture2D(framebuffer, texCoord);\n"
  "}\n";

const string VERTEX_SHADER =
  "#version 130\n"
  "\n"
  "in  vec3 position;\n"
  "out vec2 texCoord;\n"
  "\n"
  "void main() {\n"
    "texCoord    = (position.xy + vec2(1.0)) * 0.5;\n"
    "gl_Position = vec4(position, 1);\n"
  "}\n";
