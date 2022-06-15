#version 430 core

// Generated from Alexandria Common Shader Language

layout (location = 0) in vec4 acsl_vertex_input_position;

out vec4 acsl_pixel_input_position;

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
    VertexInput vertex_input = VertexInput(acsl_vertex_input_position);

    PixelInput acsl_vertex_output = PixelInput((vertex_input.position * (matrix_buffer.object * matrix_buffer.view)));
    acsl_pixel_input_position = acsl_vertex_output.position;
    gl_Position = acsl_pixel_input_position;
    return;
}


