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

uniform mat4 inverse_matrix;
uniform uvec2 frame_size;

out vec4 frag_color;

const int samples = 16;

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

const float far = 1500;
const float near = 50;

// https://www.cs.rpi.edu/~cutler/classes/advancedgraphics/S08/final_projects/lajzer_nottingham.pdf
void main(void)
{
     //get the depth
    float depth = texelFetch(depth_texture, ivec2(gl_FragCoord.xy-0.5),0).x;    // in depth buffer values
    vec3 normal = texelFetch(input_texture, ivec2(gl_FragCoord.xy-0.5),0).xyz;    // in depth buffer values
    vec3 noise = texture(noise_texture, (gl_FragCoord.xy-0.5) / frame_size).xyz;    // in depth buffer values

    float occlusion = 0.0;
    float depth_sample;
    vec2 sp;

	float z = (2.0 * near) / (far + near - depth * (far - near)); // convert to linear values 

    float radius = mix(2, 20, z);
    float samples_count = 0.0;

    for(int i=0; i < samples; i++)
    {
         //sp = radius * (reflect(sample_sphere[i], f_norm).xy) + gl_FragCoord.xy;
         vec2 ss_offset = vec2(sample_sphere[i].x * noise.x, sample_sphere[i].y * noise.y);
         sp = radius * ss_offset + gl_FragCoord.xy;
         depth_sample = depth - texelFetch(depth_texture, ivec2(sp), 0).r;

        // the lower bound makes acne dissapear. 
         if(depth_sample > 0.00001)
         {
               occlusion += step(depth_sample,0.005);
               samples_count += step(0.005, depth_sample);
         }
    }

	occlusion = 1.0 -(occlusion / (samples-samples_count));
    frag_color = vec4(occlusion, occlusion, occlusion, 1.0);

   // frag_color = vec4(
   //                 clamp(occlusion, 0.0, 0.33)*3,
   //                 (clamp(occlusion, 0.33, 0.66)-0.33)*3,
   //                 (clamp(occlusion, 0.66, 1.0)-0.66)*3.0,
   //                 1.0);
}
