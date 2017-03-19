// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- VERTEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#version 140
in vec2 position;
in vec2 tex_coords;   

smooth out vec2 coords;

void main() {
    gl_Position = vec4(position,0.0, 1.0); 
    coords = tex_coords;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// <- FRAGMENT ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#version 140
uniform sampler2D quad_texture;
uniform bool is_depth;

smooth in vec2 coords;
out vec4 frag_color;

void main() {
    if (is_depth){

        // 1500 range?
        const float far = 1500;
        const float near = 50;

        float z = texture(quad_texture, coords).r;
        float v = (2.0 * near) / (far + near - z * (far - near));  // convert to linear values 
         
        frag_color = vec4(
                clamp(v, 0.0, 0.33)*3,
                (clamp(v, 0.33, 0.66)-0.33)*3,
                (clamp(v, 0.66, 1.0)-0.66)*3.0,
                1.0);
    }
    else
    {
        frag_color = texture(quad_texture, coords);
    }
}
