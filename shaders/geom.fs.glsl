#version 330

flat in vec4 fColor;
flat in uint text_id;
in vec2 tex_coords;

out vec4 color;

 uniform sampler2D tex1;
 uniform sampler2D tex2;
 uniform sampler2D tex3;

 uniform sampler2D tex4;
 uniform sampler2D tex5;
 uniform sampler2D tex6;

void main() {

    switch(text_id)
    {
        case 0u: 
            color = texture(tex1, tex_coords);
        case 1u:                   
            color = texture(tex2, tex_coords);
        case 2u:                   
            color = texture(tex3, tex_coords);
        case 3u:                   
            color = texture(tex4, tex_coords);
        case 4u:                   
            color = texture(tex5, tex_coords);
        case 5u:                   
            color = texture(tex6, tex_coords);
        default:                  
            color = texture(tex1, tex_coords);
    }
}
