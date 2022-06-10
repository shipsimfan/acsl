// Generated from Alexandria Common Shader Language

struct VertexInput {
    float4 position: POSITION;
};

struct PixelInput {
    float4 position: SV_POSITION;
};

PixelInput vertex_main(VertexInput input) {
    return PixelInput(input.position);
}

float4 fragment_main(PixelInput input) : SV_TARGET {
    return float4(1.0, 1.0, 0.0, 1.0);
}

