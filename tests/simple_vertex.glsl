#version 330 core

// Generated from Alexandria Common Shader Language

layout (location = 0) in vec4 acsl_vertex_input_position;

out vec4 acsl_pixel_input_position;

struct VertexInput {
    vec4 position;
};

struct PixelInput {
    vec4 position;
};

void main() {
    VertexInput vertex_input = VertexInput(acsl_vertex_input_position);

    PixelInput acsl_vertex_output = PixelInput(vertex_input.position);
    acsl_pixel_input_position = acsl_vertex_output.position;
    gl_Position = acsl_pixel_input_position;
    return;
}


