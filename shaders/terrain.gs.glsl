#version 410 core

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Geometry shader 
layout (triangles) in;
layout (triangle_strip) out;


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

void main(){
}
