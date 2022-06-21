// Generated from Alexandria Common Shader Language

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

static const float4 PIXEL_COLOR = float4(1.0, 1.0, 1.0, 1.0);

PixelInput vertex_main(VertexInput vertex_input) {
    return acsl_create_PixelInput(vertex_input.position);
}

float4 fragment_main(PixelInput pixel_input) : SV_TARGET {
    return PIXEL_COLOR;
}

