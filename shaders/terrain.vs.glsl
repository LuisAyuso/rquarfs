#version 410 core

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat4 light_space_matrix;

uniform sampler2D atlas_texture;
uniform sampler2D shadow_texture;
uniform uint atlas_side;
uniform vec3 sun_pos;
uniform vec3 cam_pos;
uniform bool shadows_enabled;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

layout (location = 0) in vec3 position;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

    vec4 vertex_real_coords = vec4(position, 1.0);

	vec4 vertex_modelspace = model * vertex_real_coords;
	gl_Position =  perspective * view * vertex_modelspace;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
