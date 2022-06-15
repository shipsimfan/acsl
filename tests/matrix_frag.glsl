#version 430 core

// Generated from Alexandria Common Shader Language

out vec4 acsl_fragment_color;

in vec4 acsl_pixel_input_position;

struct MatrixBuffer {
    mat4x4 object;
    mat4x4 view;
};

struct VertexInput {
    vec4 position;
};

struct PixelInput {
    vec4 position;
};

layout(location = 0) uniform MatrixBuffer matrix_buffer;


void main() {
    PixelInput pixel_input = PixelInput(acsl_pixel_input_position);

    acsl_fragment_color = vec4(1.0, 1.0, 1.0, 1.0);
    return;
}

