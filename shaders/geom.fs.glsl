#version 330

flat in vec4 fColor;
in vec2 texture_out;

out vec4 color;

uniform sampler2D tex_atlas;
uniform uint atlas_side;

void main() {

	float cell_side = 1.0 / float(atlas_side);
    color = texture(tex_atlas, texture_out);
}
