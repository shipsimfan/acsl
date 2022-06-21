// Generated from Alexandria Common Shader Language

struct MatrixBuffer {
    float4x4 object;
    float4x4 view;
};
MatrixBuffer acsl_create_MatrixBuffer(float4x4 object, float4x4 view) {
    MatrixBuffer output;
    output.object = object;
    output.view = view;
    return output;
}

struct VertexInput {
    float4 position: POSITION;
    float4 color: COLOR;
};
VertexInput acsl_create_VertexInput(float4 position, float4 color) {
    VertexInput output;
    output.position = position;
    output.color = color;
    return output;
}

struct PixelInput {
    float4 position: SV_POSITION;
    float4 color: COLOR;
};
PixelInput acsl_create_PixelInput(float4 position, float4 color) {
    PixelInput output;
    output.position = position;
    output.color = color;
    return output;
}

cbuffer acsl_constant_buffer_0 : register(b0) {
    MatrixBuffer matrix_buffer;
}

PixelInput vertex_main(VertexInput vertex_input) {
    float4 output_position = mul(vertex_input.position, mul(matrix_buffer.object, matrix_buffer.view));
    return acsl_create_PixelInput(output_position, vertex_input.color);
}

float4 fragment_main(PixelInput pixel_input) : SV_TARGET {
    return pixel_input.color;
}

