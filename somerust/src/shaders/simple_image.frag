#version 330 core

in VS_OUTPUT {
    vec2 vs_tex_vert;
} IN;

uniform sampler2D tex;

out vec4 Color;

void main(void) {
  vec4 fragClr = texture2D(tex, IN.vs_tex_vert);
  if (fragClr.a > 0.0) {
    Color = fragClr;
  } else {
    discard;
  }
}