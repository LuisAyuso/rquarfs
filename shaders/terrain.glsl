// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- VERTEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ 
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

layout (location = 0) in uvec2 position;     
            // ------- from here on are instanciated
layout (location = 1) in uvec2 tile_offset;  // comes from the instance attributes

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {
    gl_Position = vec4(position.x + (tile_offset.x*64), 0.0, 
                       position.y + (tile_offset.y*64), 1.0);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- TESSELLATION_CONTROL ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 410 core

layout (vertices = 4) out;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

void main()
{
	 gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

    if (gl_InvocationID == 0) {

        gl_TessLevelInner[0] = 64;
        gl_TessLevelInner[1] = 64;

        gl_TessLevelOuter[0] = 64;
        gl_TessLevelOuter[1] = 64;
        gl_TessLevelOuter[2] = 64;
        gl_TessLevelOuter[3] = 64;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- TESSELLATION_EVALUATION ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 410 core

// am I rendereing everithing ccw?  it seems that I do
layout(quads, fractional_even_spacing, ccw) in;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
uniform sampler2D height_map;
uniform uvec2 height_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out float height; 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

// triangles:
//	vec3 position = vec3(gl_TessCoord.x) * gl_in[0].gl_Position.xyz +
//					vec3(gl_TessCoord.y) * gl_in[1].gl_Position.xyz +
//					vec3(gl_TessCoord.z) * gl_in[2].gl_Position.xyz;
//	gl_Position =  perspective * view * vec4(position, 1.0);

// quads:
    float u = gl_TessCoord.x;
    float v = gl_TessCoord.y;

    vec4 a = mix(gl_in[0].gl_Position, gl_in[1].gl_Position, u);
    vec4 b = mix(gl_in[3].gl_Position, gl_in[2].gl_Position, u);
    vec4 position = mix(a, b, v);

    // interpolated 
    vec2 texcoord = vec2(position.x / height_size.x, position.z / height_size.y);
    int tmp = int(texture(height_map, texcoord).r *128);

    // fetch one texel... this is more like it but is not working
    //ivec2 texcoord = ivec2(position.x / height_size.x, position.z / height_size.y);
    //int tmp = int(texelFetch(height_map, texcoord,1).r *128);

    position.y = height = float(tmp);
    gl_Position = vec4(position.xyz,1.0);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- GEOMETRY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 410 core

layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;

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

in float height[];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {
    vec4 pos0 =  gl_in[0].gl_Position;
    vec4 pos1 =  gl_in[1].gl_Position;
    vec4 pos2 =  gl_in[2].gl_Position;

    float a = pos0.y;
    float b = pos1.y;
    float c = pos2.y;

    float h = max(height[0], max(height[1], height[2]));
    pos0.y = pos1.y = pos2.y = h;

    gl_Position = perspective * view * model * pos0;
    EmitVertex();
    gl_Position = perspective * view * model * pos1;
    EmitVertex();
    gl_Position = perspective * view * model * pos2;
    EmitVertex();

    EndPrimitive();
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- FRAGMENT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
out vec4 color; 
void main() {
    color = vec4(0.0, 0.0, 0.0, 0.0);
}
