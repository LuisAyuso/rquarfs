#version 330 core

uniform mat4 perspective_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;
uniform mat4 light_bias;

uniform sampler2D atlas_texture;
uniform sampler2D shadow_texture;
uniform uint atlas_side;
uniform vec3 sun_pos;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

flat   in vec4 face_normal;
smooth in vec2 texture_coords;
smooth in vec4 vertex_coord;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

out vec4 frag_color;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

void main()
{
    vec4 rgba = texture(atlas_texture, texture_coords);// * fColor;
    ////vec4 rgba = texture(shadow_texture, texture_coords);// * fColor;

	float vis = 0.0;
	if ( texture( shadow_texture, gl_FragCoord.xy).z  < gl_FragCoord.z){
		frag_color = vec4(0.2, 0.0, 0.4, 1.0);
		return;
	}

	frag_color =vec4(texture( shadow_texture, gl_FragCoord.xy).x  , .4, .2, 1.0);

//	frag_color = 0.2 * vec4( texture( shadow_texture, gl_FragCoord.xy ));
//	//frag_color = 0.2 * vec4( texture( shadow_texture, texture_coords));

	vec3 normalDirection = normalize(face_normal.xyz);
	vec3 lightDirection = normalize(sun_pos);

	vec3 scene_ambient = vec3(0.1, 0.1, 0.1);
	vec3 ambientLighting = vec3(scene_ambient) * rgba.xyz;

	vec3 diffuseReflection = vec3(vis, vis, vis) * rgba.xyz * max(0.0, dot(normalDirection, lightDirection));

    //frag_color = vec4(diffuseReflection + ambientLighting, 1.0);
}
