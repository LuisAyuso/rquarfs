#version 330 core

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

uniform sampler2D tex_atlas;
uniform sampler2D shadow_texture;
uniform uint atlas_side;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coord;
layout (location = 3) in vec3 world_position;
layout (location = 4) in vec3 in_color;      
layout (location = 5) in vec2 tex_offset;      

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

flat   out vec4 face_normal;
smooth out vec2 texture_coords;
smooth out vec4 vertex_coord;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec4 vertex_modelspace = vec4(position + world_position, 1.0);
	gl_Position = vertex_coord =  perspective * view * model * vertex_modelspace;

	texture_coords = clamp(tex_coord, 0.05, 0.95) / float(atlas_side);
    texture_coords = tex_offset + texture_coords;

    face_normal = normalize(vec4(normal, 1.0));
}
