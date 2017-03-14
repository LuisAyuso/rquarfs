// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- VERTEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ 
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#version 410 core

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // ------- the static geometry. just a quad of side 64
layout (location = 0) in uvec2 position;     
            // ------- from here on are instanciated
layout (location = 1) in uvec3 tile_offset;  // comes from the instance attributes

out uint vs_mintess;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {
    gl_Position = vec4(position.x + (tile_offset.x*64), 0.0, 
                       position.y + (tile_offset.y*64), 1.0);
    vs_mintess = tile_offset.z;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- TESSELLATION_CONTROL ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 410 core

layout (vertices = 4) out;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

uniform mat4 model;
uniform mat4 view;
uniform mat4 pvm;

uniform sampler2D height_map;
uniform uvec2 height_size;
uniform vec3 cam_pos;
uniform uvec2 screen_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~
in uint vs_mintess[];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

vec4 project(vec4 vertex){
    vec2 texcoord = vec2(vertex.x / height_size.x, vertex.z / height_size.y);
    vertex.y = int(texture(height_map, texcoord).r *256);
    vec4 result = pvm * vertex;
    result /= result.w;
    return result;
}

bvec2 or(bvec2 a, bvec2 b){
    return bvec2(a.x || b.x, a.y || b.y);
}

bool offscreen(vec4 vertex){
    if(vertex.z < -0.5){
        return true;
    }   
    return any(or(
        lessThan(vertex.xy, vec2(-1.7)),
        greaterThan(vertex.xy, vec2(1.7)))
    );  
}

float distance_to_camera(vec4 vertex, vec3 camera){
    vec2 texcoord = vec2(vertex.x / height_size.x, vertex.z / height_size.y);
    vertex.y = int(texture(height_map, texcoord).r *256);
    vec4 tmp = model * vertex;
	return clamp(distance(vertex.xyz, camera.xyz) / 1500.0, 0.0, 1.0);
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

        if(all(bvec4(offscreen(v0), offscreen(v1), offscreen(v2), offscreen(v3))))
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
            float d0 = distance_to_camera(model * gl_in[0].gl_Position, cam_pos);
            float d1 = distance_to_camera(model * gl_in[1].gl_Position, cam_pos);
            float d2 = distance_to_camera(model * gl_in[2].gl_Position, cam_pos);
            float d3 = distance_to_camera(model * gl_in[3].gl_Position, cam_pos);

            float dist = min(d0, min(d1, min(d2, d3)));

            uint level = max(uint(mix(9, -1, dist)), vs_mintess[ID]);

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

uniform sampler2D height_map;
uniform uvec2 height_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out float te_height; 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

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

uniform mat4 pvm;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 gs_color;
out vec2 gs_coordinates;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

    vec4 pos[3] = vec4[](
                 gl_in[0].gl_Position,
                 gl_in[1].gl_Position,
                 gl_in[2].gl_Position
                 );
    float d0 = distance(pos[0].xz, pos[1].xz);
    float d1 = distance(pos[1].xz, pos[2].xz);
    float d2 = distance(pos[0].xz, pos[2].xz);
    float shortest_side = min(d0,min(d1, d2));


    if (shortest_side == 1.0){

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

            vec4 color1 = vec4(0.8,0.0,0.0,1.0);
            vec4 color2 = vec4(0.0,0.8,0.0,1.0);
            vec4 color3 = vec4(0.0,0.0,0.8,1.0);
            vec4 color4 = vec4(0.8,0.0,0.8,1.0);
            vec4 color5 = vec4(0.0,0.8,0.8,1.0);
            vec4 color6 = vec4(0.8,0.8,0.0,1.0);

            gl_Position = pvm * first_d;
            gs_color = vec4(0.8, 0.0, 0.0, 1.0);
            gs_coordinates = first_d.xz;
            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * origin_d;
            gs_color = vec4(0.0, 0.8, 0.0, 1.0);
            gs_coordinates = origin_d.xz;
 //           gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * first_u;
            gs_color = vec4(0.0, 0.0, 0.8, 1.0);
            gs_coordinates = first_u.xz;
//            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * origin_u;
            gs_color = vec4(0.8, 0.8, 0.0, 1.0);
            gs_coordinates = origin_u.xz;
//            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * last_u;
            gs_color = vec4(0.8, 0.8, 0.8, 1.0);
            gs_coordinates = last_u.xz;
//            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * origin_d;
            gs_color = vec4(0.0, 0.8, 0.0, 1.0);
            gs_coordinates = origin_d.xz;
//            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
            gl_Position = pvm * last_d;
            gs_color = vec4(0.0, 0.0, 0.0, 1.0);
            gs_coordinates = last_d.xz;
//            gs_color = shortest_side <= 1.5? color1: shortest_side <= 2.5? color2: color3;
            EmitVertex();
        }
        EndPrimitive();
    }
    else
    {
            gl_Position = pvm * pos[0];
            gs_coordinates = pos[0].xz;
            gs_color = vec4(0.8, 0.0, 0.0, 1.0);
            EmitVertex();
            gl_Position = pvm * pos[1];
            gs_coordinates = pos[1].xz;
            gs_color = vec4(0.0, 0.8, 0.0, 1.0);
            EmitVertex();
            gl_Position = pvm * pos[2];
            gs_coordinates = pos[2].xz;
            gs_color = vec4(0.0, 0.0, 0.8, 1.0);
            EmitVertex();
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- FRAGMENT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#version 410 core

uniform sampler2D color_map;
uniform uvec2 height_size;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

in vec4 gs_color; 
in vec2 gs_coordinates; 

out vec4 color; 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {
    vec2 texcoord = vec2(gs_coordinates.x / height_size.x, gs_coordinates.y / height_size.y);
    color = texture(color_map, texcoord);

	//color = gs_color;
}
