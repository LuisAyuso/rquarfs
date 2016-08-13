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

	//gl_Position = perspective_matrix * view_matrix * model_matrix * vec4(position, 0.0f);
	vec4 tmp = vec4(position, 0.0f) + vec4(0.0, 0.0, 0.0, 2.0);
	gl_Position = perspective_matrix * view_matrix * model_matrix * tmp;

    fColor = vec4(in_color + position, 1.0);

//    vec4 frag_position = model_matrix * vec4(position, 0.0f, 1.0f);
//    gl_Position = perspective_matrix * view_matrix * frag_position;
//    fColor = vec4(in_color + vec3(position, 0.0), 1.0);
    //float offset = gl_InstanceID *0.1;
    //gl_Position = vec4(position + world_position, 0.0f, 1.0f);
    //fColor = vec4(in_color + vec3(position, 0.0), 1.0);
}
