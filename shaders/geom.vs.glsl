#version 330

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coord;
layout (location = 3) in vec2 world_position;
layout (location = 4) in vec3 in_color;      
layout (location = 5) in uint tex;      

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

flat out vec4 fColor;
flat out uint text_id;
out vec2 tex_coords;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec4 tmp = vec4(position + vec3(world_position, 0.0), 1.0);
	gl_Position = perspective_matrix * view_matrix * model_matrix * tmp;
    tex_coords = tex_coord;

    fColor = vec4(in_color + position, 1.0);
}
