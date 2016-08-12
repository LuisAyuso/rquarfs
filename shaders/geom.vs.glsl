#version 330

layout (location = 0) in vec2 position;
layout (location = 1) in vec2  world_position; // (f32, f32),
layout (location = 2) in vec3 in_color;

out vec4 fColor;


void main() {
    float offset = gl_InstanceID *0.1;
    gl_Position = vec4(position + offset, 0.0f, 1.0f);
    fColor = vec4(in_color, 1);
}
