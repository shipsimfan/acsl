#version 430 core

// Generated from Alexandria Common Shader Language

layout (location = 0) in vec4 acsl_vertex_input_position;
layout (location = 1) in vec4 acsl_vertex_input_color;

out vec4 acsl_pixel_input_position;
out vec4 acsl_pixel_input_color;

struct MatrixBuffer {
    mat4x4 object;
    mat4x4 view;
};

struct VertexInput {
    vec4 position;
    vec4 color;
};

struct PixelInput {
    vec4 position;
    vec4 color;
};

layout(location = 0) uniform MatrixBuffer matrix_buffer;

void main() {
    VertexInput vertex_input = VertexInput(acsl_vertex_input_position, acsl_vertex_input_color);

    vec4 output_position = (vertex_input.position * (matrix_buffer.object * matrix_buffer.view));
    PixelInput acsl_vertex_output = PixelInput(output_position, vertex_input.color);
    acsl_pixel_input_position = acsl_vertex_output.position;
    acsl_pixel_input_color = acsl_vertex_output.color;
    gl_Position = acsl_pixel_input_position;
    return;
}


