struct VertexOutput {
    [[location(0)]] uv: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] index: u32) -> VertexOutput {
    let u = f32(((index + 2u) / 3u) % 2u);
    let v = f32(((index + 1u) / 3u) % 2u);
    let uv = vec2<f32>(u, v);

    let position = vec4<f32>(-1.0 + uv * 2.0, 0.0, 1.0);

    return VertexOutput(vec2<f32>(uv.x, uv.y), position);
}

struct FragmentOutput {
    [[location(0)]] frag_color: vec4<f32>;
};

// [[group(0), binding(0)]]
// var in_texture: texture_2d<f32>;
// [[group(0), binding(1)]]
// var in_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> FragmentOutput {
    // return FragmentOutput(textureSample(in_texture, in_sampler, in.uv));
    return FragmentOutput(vec4<f32>(in.uv, 0.0, 1.0));
}