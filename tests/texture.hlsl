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
    float2 uv: TEXCOORD;
};
VertexInput acsl_create_VertexInput(float4 position, float4 color, float2 uv) {
    VertexInput output;
    output.position = position;
    output.color = color;
    output.uv = uv;
    return output;
}

struct PixelInput {
    float4 position: SV_POSITION;
    float4 color: COLOR;
    float2 uv: TEXCOORD;
};
PixelInput acsl_create_PixelInput(float4 position, float4 color, float2 uv) {
    PixelInput output;
    output.position = position;
    output.color = color;
    output.uv = uv;
    return output;
}

ConstantBuffer<MatrixBuffer> matrix_buffer : register(b0);

Texture2D tex : register(t0);
SamplerState acsl_tex_sampler_state : register(s0);

PixelInput vertex_main(VertexInput vertex_input) {
    float4 output_position = mul(vertex_input.position, mul(matrix_buffer.object, matrix_buffer.view));
    return acsl_create_PixelInput(output_position, vertex_input.color, vertex_input.uv);
}

float4 fragment_main(PixelInput pixel_input) : SV_TARGET {
    return mul(pixel_input.color, tex.Sample(acsl_tex_sampler_state, pixel_input.uv));
}

