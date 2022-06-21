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
};
VertexInput acsl_create_VertexInput(float4 position) {
    VertexInput output;
    output.position = position;
    return output;
}

struct PixelInput {
    float4 position: SV_POSITION;
};
PixelInput acsl_create_PixelInput(float4 position) {
    PixelInput output;
    output.position = position;
    return output;
}

cbuffer acsl_constant_buffer_0 : register(b0) {
    MatrixBuffer matrix_buffer;
}

PixelInput vertex_main(VertexInput vertex_input) {
    return acsl_create_PixelInput(mul(vertex_input.position, mul(matrix_buffer.object, matrix_buffer.view)));
}

float4 fragment_main(PixelInput pixel_input) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}

