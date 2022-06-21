#version 430 core

// Generated from Alexandria Common Shader Language

out vec4 acsl_fragment_color;

in vec4 acsl_pixel_input_position;
in vec4 acsl_pixel_input_color;
in vec2 acsl_pixel_input_uv;

struct MatrixBuffer {
    mat4x4 object;
    mat4x4 view;
};

struct VertexInput {
    vec4 position;
    vec4 color;
    vec2 uv;
};

struct PixelInput {
    vec4 position;
    vec4 color;
    vec2 uv;
};

layout(location = 0) uniform MatrixBuffer matrix_buffer;

layout(location = 32) uniform sampler2D tex;


void main() {
    PixelInput pixel_input = PixelInput(acsl_pixel_input_position, acsl_pixel_input_color, acsl_pixel_input_uv);

    acsl_fragment_color = (pixel_input.color * texture(tex, pixel_input.uv));
    return;
}

