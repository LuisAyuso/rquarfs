#version 330

flat in vec4 fColor;
flat in uint tex_id;
     in vec2 tex_coords;

out vec4 color;

 uniform sampler2D tex1;
 uniform sampler2D tex2;
 uniform sampler2D tex3;

 uniform sampler2D tex4;
 uniform sampler2D tex5;
 uniform sampler2D tex6;

void main() {

    int x = int(tex_id);

    switch(x)
    {
        case 0: 
            color = texture(tex2, tex_coords);
        case 1:                   
            color = texture(tex3, tex_coords);
        case 2:                   
            color = texture(tex4, tex_coords);
        case 3:                   
            color = texture(tex5, tex_coords);
        case 4:                   
            color = texture(tex6, tex_coords);
        case 5:                   
            color = texture(tex1, tex_coords);
        default:                  
            color = texture(tex2, tex_coords);
    }
}
