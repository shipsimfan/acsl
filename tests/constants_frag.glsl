#version 430 core

// Generated from Alexandria Common Shader Language

out vec4 acsl_fragment_color;

in vec4 acsl_pixel_input_position;

struct VertexInput {
    vec4 position;
};

struct PixelInput {
    vec4 position;
};

const vec4 PIXEL_COLOR = vec4(1.0, 1.0, 1.0, 1.0);


void main() {
    PixelInput pixel_input = PixelInput(acsl_pixel_input_position);

    acsl_fragment_color = PIXEL_COLOR;
    return;
}

