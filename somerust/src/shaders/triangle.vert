#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(Position, 1.0);
    OUT.Color = Color;
}