#version 330

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

uniform sampler2D tex_atlas;
uniform uint atlas_side;
uniform vec3 sun_pos; 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

flat   in vec4 face_normal;
smooth in vec2 texture_out;
smooth in vec4 vertex_coord;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 frag_color;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main()
{

    vec4 rgba = texture(tex_atlas, texture_out);// * fColor;

	frag_color = vec4(normalize(sun_pos), 1.0);

	vec3 normalDirection = normalize(face_normal.xyz);
	vec3 lightDirection = normalize(sun_pos);

	vec3 scene_ambient = vec3(0.1, 0.1, 0.1);
	vec3 ambientLighting = vec3(scene_ambient) * rgba.xyz;

	vec3 diffuseReflection = vec3(0.9, 0.9, 0.9) * rgba.xyz * max(0.0, dot(normalDirection, lightDirection));

    frag_color = vec4(diffuseReflection + ambientLighting, 1.0);
}
