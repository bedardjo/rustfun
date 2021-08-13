#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 mvp;

out VS_OUTPUT {
  vec2 vs_tex_vert;
} OUT;

void main() {
   gl_Position = mvp * vec4(Position, 1.0);
   OUT.vs_tex_vert = TexCoord;
}
