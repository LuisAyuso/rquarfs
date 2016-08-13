#version 330

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 world_position; // (f32, f32),
layout (location = 2) in vec3 in_color;      // (f32, f32)

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 fColor;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec4 tmp = vec4(position + vec3(world_position, 0.0), 1.0) 
	gl_Position = perspective_matrix * view_matrix * model_matrix * tmp;

    fColor = vec4(in_color + position, 1.0);
}
