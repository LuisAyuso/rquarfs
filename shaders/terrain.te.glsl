#version 410 core

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Tesselation evaluation
layout(quads, equal_spacing, cw) in;

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

void main(void)
{
//    // interpolate in horizontal direction between vert. 0 and 3
//    vec3 p0 = mix(tcPosition[0], tcPosition[3], gl_TessCoord.x);
//    // interpolate in horizontal direction between vert. 1 and 2
//    vec3 p1 = mix(tcPosition[1], tcPosition[2], gl_TessCoord.x);
//    // interpolate in vert direction
//    vec3 p = mix(p0, p1, gl_TessCoord.y);
//    tePatchDistance = gl_TessCoord.xy;
//    tePosition = normalize(p); // project on unit sphere
//    gl_Position = Projection * Modelview * vec4(tePosition, 1);
}
