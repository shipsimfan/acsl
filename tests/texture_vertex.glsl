#version 430 core

// Generated from Alexandria Common Shader Language

layout (location = 0) in vec4 acsl_vertex_input_position;
layout (location = 1) in vec4 acsl_vertex_input_color;
layout (location = 2) in vec2 acsl_vertex_input_uv;

out vec4 acsl_pixel_input_position;
out vec4 acsl_pixel_input_color;
out vec2 acsl_pixel_input_uv;

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
    VertexInput vertex_input = VertexInput(acsl_vertex_input_position, acsl_vertex_input_color, acsl_vertex_input_uv);

    vec4 output_position = (vertex_input.position * (matrix_buffer.object * matrix_buffer.view));
    PixelInput acsl_vertex_output = PixelInput(output_position, vertex_input.color, vertex_input.uv);
    acsl_pixel_input_position = acsl_vertex_output.position;
    acsl_pixel_input_color = acsl_vertex_output.color;
    acsl_pixel_input_uv = acsl_vertex_output.uv;
    gl_Position = acsl_pixel_input_position;
    return;
}


