#version 330

flat   in vec4 face_normal;
smooth in vec2 texture_out;

out vec4 color;

uniform sampler2D tex_atlas;
uniform uint atlas_side;

void main() {
    color = texture(tex_atlas, texture_out);// * fColor;
}
