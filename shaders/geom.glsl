// <- VERTEX


#version 330 core

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

            // ------- first geomtry 
layout (location = 0) in vec3 position;     // comes from the geometry
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coord;
            // ------- from here on are instanciated
layout (location = 3) in vec3 world_position;  // comes from the instance attributes
layout (location = 4) in vec3 in_color;      
layout (location = 5) in vec2 tex_offset;      
layout (location = 6) in float vox_height;      

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

flat   out vec4 face_normal;
smooth out vec2 texture_coords;
flat out vec4 vertex_modelspace;
smooth out vec4 frag_lightSpace_coords;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main() {

	vec3 tmp = vec3(position.x, position.y*vox_height, position.z);
    vec4 vertex_real_coords = vec4(tmp + world_position, 1.0);

	vertex_modelspace = model * vertex_real_coords;
	gl_Position =  perspective * view * vertex_modelspace;

	texture_coords = clamp(tex_coord, 0.05, 0.95) / float(atlas_side);
    texture_coords = tex_offset + texture_coords;

    face_normal = normalize(vec4(normal, 1.0));

    frag_lightSpace_coords = light_space_matrix * vertex_modelspace;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- FRAGMENT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#version 330 core

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

flat   in vec4 face_normal;
smooth in vec2 texture_coords;
flat in vec4 vertex_modelspace;
smooth in vec4 frag_lightSpace_coords;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 frag_color;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
vec2 poissonDisk[4] = vec2[](
  vec2( -0.94201624, -0.39906216 ),
  vec2( 0.94558609, -0.76890725 ),
  vec2( -0.094184101, -0.92938870 ),
  vec2( 0.34495938, 0.29387760 )
);

float ShadowCalculation(vec4 fragPosLightSpace, float bias)
{

    //return 0.0;
    // perform perspective divide
    vec3 projCoords = fragPosLightSpace.xyz; // / fragPosLightSpace.w;

    // Transform to [0,1] range
    projCoords = projCoords * 0.5 + 0.5;
    // Get closest depth value from light's perspective (using [0,1] range fragPosLight as coords)
    float closestDepth = texture(shadow_texture, projCoords.xy).r; 
    // Get depth of current fragment from light's perspective
    float currentDepth = projCoords.z;
	float shadow = 0.0;

	// this performs some kind of soft shadow
	// poisson sampling
	for (int i=0;i<4;i++){
      // Check whether current frag pos is in shadow
	  if ( texture( shadow_texture, projCoords.xy + poissonDisk[i]/700.0 ).r  <  currentDepth - bias ){
		shadow+=0.25;
	  }
	}

    return shadow;
}  

void main()
{
    vec3 color = texture(atlas_texture, texture_coords ).rgb;
    vec3 normal = normalize(face_normal.xyz);
    vec3 lightColor = vec3(1.0);

    // Ambient
    vec3 ambient = 0.15 * color;

    // Diffuse
    vec3 sun_pos2 = (vec4(sun_pos, 1.0)* model).xyz;
    vec3 lightDir = normalize(sun_pos2 - vertex_modelspace.xyz);
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = diff * lightColor;

    // Specular
    vec3 cam_pos2 = (vec4(cam_pos, 1.0)* model).xyz;
    vec3 viewDir = normalize(cam_pos2 - vertex_modelspace.xyz);
    float spec = 0.0;
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    spec = pow(max(dot(normal, halfwayDir), 0.0), 64.0);
    vec3 specular = spec * lightColor;    

    vec3 lighting;

    // Calculate shadow
    if (shadows_enabled)
    {
        float bias = max(0.05 * (1.0 - dot(normal, lightDir)), 0.005);  
        float shadow = ShadowCalculation(frag_lightSpace_coords, bias);       
        lighting = (ambient + (1-shadow) * (diffuse + specular)) * color;    
    }
    else
    {
        lighting = color;    
    }
    
    frag_color = vec4(lighting, 1.0f);
}
