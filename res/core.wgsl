//Vertex
struct VertexInput {
    [[location(0)]] pos : vec3<f32>;
    [[location(1)]] uv  : vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_pos : vec4<f32>;
};

[[stage(vertex)]]
fn vertex_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(vertex.pos, 1.0);
    
    return out;
}

// Fragment
[[stage(fragment)]]
fn fragment_main(vertex: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}