// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- VERTEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 330

in vec2 position;


void main() {
    gl_Position = vec4(position,0.0, 1.0); 
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- FRAGMENT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#version 330

uniform sampler2D input_texture;
uniform sampler2D depth_texture;
uniform sampler2D noise_texture;

uniform mat4 perspective;
uniform mat4 view;
uniform uvec2 frame_size;

out vec4 frag_color;

const int samples = 16;
const float CAP_MIN_DISTANCE = 0.0001;
const float CAP_MAX_DISTANCE = 0.005;

vec3 sample_sphere[samples] = vec3[](
    vec3( 0.5381, 0.1856,-0.4319), vec3( 0.1379, 0.2486, 0.4430),
    vec3( 0.3371, 0.5679,-0.0057), vec3(-0.6999,-0.0451,-0.0019),
    vec3( 0.0689,-0.1598,-0.8547), vec3( 0.0560, 0.0069,-0.1843),
    vec3(-0.0146, 0.1402, 0.0762), vec3( 0.0100,-0.1924,-0.0344),
    vec3(-0.3577,-0.5301,-0.4358), vec3(-0.3169, 0.1063, 0.0158),
    vec3( 0.0103,-0.5869, 0.0046), vec3(-0.0897,-0.4940, 0.3287),
    vec3( 0.7119,-0.0154,-0.0918), vec3(-0.0533, 0.0596,-0.5411),
    vec3( 0.0352,-0.0631, 0.5460), vec3(-0.4776, 0.2847,-0.0271)
);
  
const float radius = 0.0002;
const float area = 0.0075;
const float falloff = 0.000001;

const float far = 1500;
const float near = 50;

//https://github.com/McNopper/OpenGL/blob/master/Example28/shader/ssao.frag.glsl
void main(){

    float z = texelFetch(depth_texture, ivec2(gl_FragCoord.xy-0.5),0).x;    // in depth buffer values
    float depth = (2.0 * near) / (far + near - z * (far - near));  // convert to linear values 
    //frag_color = vec4(depth, 0.0, 0.0, 1.0);
    //return;

    vec4 position = vec4(gl_FragCoord.xy-0.5, depth, 1.0);                      // screen pos + depth
    // frag_color = vec4(normal, 1.0);

    vec3 normal = texelFetch(input_texture, ivec2(gl_FragCoord.xy-0.5),0).xyz*2.0 -1.0; // -1,1 range
    // frag_color = vec4(normal, 1.0);
    
    vec2 noise_coords =  vec2((gl_FragCoord.x-0.5) /frame_size.x,(gl_FragCoord.y-0.5) /frame_size.y);
    //vec3 noise = texture(noise_texture, noise_coords).xyz;         // normal random vectors
    vec3 noise = vec3(1.0);
    // frag_color = vec4(noise, 1.0);

    vec3 tangent = normalize(noise - dot(noise, normal) * normal);  // magic
    vec3 bitangent = cross(normal, tangent);

    frag_color = vec4(tangent, 1.0);

    mat3 kernelMatrix = mat3(tangent, bitangent, normal);    // this matrix orientates samples  according to vector

    float occlusion = 0.0;
    for (int i = 0; i < samples; i++){
        vec3 sampleVector = ( kernelMatrix * sample_sphere[i]);
                
        vec4 samplePoint = position + vec4(sampleVector, 0.0);
        
        float z = texture(depth_texture, samplePoint.xy / frame_size).x;
        float where_it_shoud_be = (2.0 * near) / (far + near - z * (far - near));  // convert to linear values 

      //float delta = depth- where_it_shoud_be;
        
        float delta = samplePoint.z - where_it_shoud_be;
        occlusion += step(0.0,delta);
    }
    occlusion /= samples;
    //occlusion = 1.0 - occlusion;
    //frag_color = vec4(occlusion, occlusion, occlusion, 1.0);
    frag_color = vec4(
                    clamp(occlusion, 0.0, 0.33)*3,
                    (clamp(occlusion, 0.33, 0.66)-0.33)*3,
                    (clamp(occlusion, 0.66, 1.0)-0.66)*3.0,
                    1.0);
}
