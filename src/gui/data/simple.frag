#version 130

precision highp float;

in  vec2 texCoord;
out vec4 fragmentColor;

uniform sampler2D framebuffer;
uniform int framebufferSamples;

void main() {
  fragmentColor = (1.0 / float(framebufferSamples)) * texture2D(framebuffer, texCoord);
}
