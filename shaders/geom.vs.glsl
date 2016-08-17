#version 330

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

uniform sampler2D tex_atlas;
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
smooth out vec2 texture_out;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec4 tmp = vec4(position + world_position, 1.0);
	gl_Position = perspective_matrix * view_matrix * model_matrix * tmp;

	texture_out = clamp(tex_coord, 0.05, 0.95) / float(atlas_side);
    texture_out = tex_offset + texture_out;

    face_normal = vec4(normal, 1.0);
}
