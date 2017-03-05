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
uniform vec3 cam_pos;

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
uniform mat4 pvm;
uniform mat4 light_space_matrix;

uniform sampler2D atlas_texture;
uniform sampler2D shadow_texture;

uniform sampler2D height_map;
uniform uvec2 height_size;

uniform vec3 cam_pos;
uniform uvec2 screen_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

vec4 project(vec4 vertex){
    vec2 texcoord = vec2(vertex.x / height_size.x, vertex.z / height_size.y);
    vertex.y = int(texture(height_map, texcoord).r *256);
    vec4 result = pvm * vertex;
    result /= result.w;
    return result;
}

bool offscreen(vec4 vertex){
    if(vertex.z < -0.5){
        return true;
    }   
    return any(
        lessThan(vertex.xy, vec2(-1.7)) ||
        greaterThan(vertex.xy, vec2(1.7))
    );  
}

float distance_to_camera(vec4 vertex){
	return clamp(distance(vertex.xyz, cam_pos) / 1000.0, 0.0, 1.0);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main()
{
     #define ID gl_InvocationID
	 gl_out[ID].gl_Position = gl_in[ID].gl_Position;

    if (ID == 0) {
         vec4 v0 = project(gl_in[0].gl_Position);
         vec4 v1 = project(gl_in[1].gl_Position);
         vec4 v2 = project(gl_in[2].gl_Position);
         vec4 v3 = project(gl_in[3].gl_Position);

       // gl_TessLevelInner[0] = 64;
       // gl_TessLevelInner[1] = 64;

       // gl_TessLevelOuter[0] = 64;
       // gl_TessLevelOuter[1] = 64;
       // gl_TessLevelOuter[2] = 64;
       // gl_TessLevelOuter[3] = 64;

		if(all(bvec4(offscreen(v0), offscreen(v1), 
		offscreen(v2), offscreen(v3))))
		{
			gl_TessLevelInner[0] = 0;
			gl_TessLevelInner[1] = 0;
			gl_TessLevelOuter[0] = 0;
			gl_TessLevelOuter[1] = 0;
			gl_TessLevelOuter[2] = 0;
			gl_TessLevelOuter[3] = 0;
		}
		else
		{
			float d0 = distance_to_camera(v0);
			float d1 = distance_to_camera(v0);
			float d2 = distance_to_camera(v0);
			float d3 = distance_to_camera(v0);

			float dist = min(d0, min(d1, min(d2, d3)));


			float level = mix(8, 0, dist);


			gl_TessLevelInner[0] = 1<<int(level);
			gl_TessLevelInner[1] = 1<<int(level);
			gl_TessLevelOuter[0] = 1<<int(level);
			gl_TessLevelOuter[1] = 1<<int(level);
			gl_TessLevelOuter[2] = 1<<int(level);
			gl_TessLevelOuter[3] = 1<<int(level);
		}
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
uniform vec3 cam_pos;
uniform sampler2D height_map;
uniform uvec2 height_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out float te_height; 

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
    int tmp = int(texture(height_map, texcoord).r *256);

    position.y = float(tmp);
    gl_Position = vec4(position.xyz,1.0);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- GEOMETRY ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 410 core

layout(triangles) in;
layout(triangle_strip, max_vertices=7) out;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat4 pvm;
uniform mat4 light_space_matrix;

uniform sampler2D atlas_texture;
uniform sampler2D shadow_texture;
uniform vec3 cam_pos;
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 gs_color;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec4 pos[3] = vec4[](
                 gl_in[0].gl_Position,
                 gl_in[1].gl_Position,
                 gl_in[2].gl_Position
				 );

    float h0 = pos[0].y;
    float h1 = pos[1].y;
    float h2 = pos[2].y;

	// we identify the common corner, and take the height from there
	// angle must not be 90, we look for the 45 ones
	//      a --- b
	//      |   /
	//      |  /
	//      | /
	//      c
	// geting height value from a may not match the complementary triangle height.
	// by getting the max between b and c, we can make sure that the complementary 
	// triangle will be at the same level
	float angle0 = dot( normalize(pos[2].xz-pos[0].xz), normalize(pos[1].xz-pos[0].xz) );
	float angle1 = dot( normalize(pos[2].xz-pos[1].xz), normalize(pos[0].xz-pos[1].xz) );
	float angle2 = dot( normalize(pos[0].xz-pos[2].xz), normalize(pos[1].xz-pos[2].xz) );

	// compute the heght of the lowest corner
	float ha = angle0 > 0? h0: max(h1,h2);
	float hb = angle1 > 0? h1: max(h0,h2);
	float h = max(ha, hb);
	float depth  = min(h0, min(h1, h2));

	// this will be cap triangle.
    pos[0].y = pos[1].y = pos[2].y = h;

	// whatever we do, we do not want to draw a trinagle starting by the 90 degrees corner
	// we want the second vertex to be the right angle. This makes easyier to render the 
	// side faces
	uint begin = angle0 == 0? 2: angle1 == 0? 0: 1;

	{
		vec4 origin_u = pos[(begin +1) %3];;
		vec4 origin_d = origin_u;
		origin_d.y = depth; 

		vec4 first_u = pos[begin];;
		vec4 first_d = first_u;
		first_d.y = depth; 

		vec4 last_u = pos[(begin+2)%3];;
		vec4 last_d = last_u;
		last_d.y = depth; 

		gl_Position = pvm * first_d;
		gs_color = vec4(0.8, 0.0, 0.0, 1.0);
		EmitVertex();
		gl_Position = pvm * origin_d;
		gs_color = vec4(0.0, 0.8, 0.0, 1.0);
		EmitVertex();
		gl_Position = pvm * first_u;
		gs_color = vec4(0.0, 0.0, 0.8, 1.0);
		EmitVertex();
		gl_Position = pvm * origin_u;
		gs_color = vec4(0.8, 0.8, 0.0, 1.0);
		EmitVertex();
		gl_Position = pvm * last_u;
		gs_color = vec4(0.8, 0.8, 0.8, 1.0);
		EmitVertex();
		gl_Position = pvm * origin_d;
		gs_color = vec4(0.0, 0.8, 0.0, 1.0);
		EmitVertex();
		gl_Position = pvm * last_d;
		gs_color = vec4(0.0, 0.0, 0.0, 1.0);
		EmitVertex();
	}
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
uniform vec3 cam_pos;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
in vec4 gs_color; 
out vec4 color; 
void main() {

    //vec3 coolColorMod = u_coolColor + u_objectColor * u_alpha;
	//vec3 warmColorMod = u_warmColor + u_objectColor * u_beta;

	color = gs_color;
    //color = vec4(0.0, 0.0, 0.0, 0.0);
}
