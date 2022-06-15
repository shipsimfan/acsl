// Generated from Alexandria Common Shader Language

struct MatrixBuffer {
    float4x4 object;
    float4x4 view;
};

struct VertexInput {
    float4 position: POSITION;
};

struct PixelInput {
    float4 position: SV_POSITION;
};

ConstantBuffer<MatrixBuffer> matrix_buffer : register(b0);

PixelInput vertex_main(VertexInput vertex_input) {
    return PixelInput(mul(vertex_input.position, mul(matrix_buffer.object, matrix_buffer.view)));
}

float4 fragment_main(PixelInput pixel_input) : SV_TARGET {
    return float4(1.0, 1.0, 1.0, 1.0);
}

