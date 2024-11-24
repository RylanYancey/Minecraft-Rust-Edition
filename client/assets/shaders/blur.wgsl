
#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> blur_strength: i32;
@group(2) @binding(1) var material_image: texture_2d<f32>;
@group(2) @binding(2) var material_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // do nothing if blur is disabled.
    if blur_strength == 0 {
        return textureSample(material_image, material_sampler, mesh.uv);
    }

    var color: vec4<f32> = vec4(0.0);
    var uv = mesh.uv;
    var texel_size: vec2<f32> = vec2<f32>(1.0) / vec2<f32>(textureDimensions(material_image));
    var sample_count: i32 = (2 * blur_strength + 1) * (2 * blur_strength + 1);
    var normalization_factor: f32 = 1.0 / f32(sample_count);

    for (var x: i32 = -blur_strength; x <= blur_strength; x++) {
        for (var y: i32 = -blur_strength; y <= blur_strength; y++) {
            var offset: vec2<f32> = vec2<f32>(f32(x), f32(y)) * texel_size;
            var sample_uv = uv + offset;
            sample_uv = clamp(sample_uv, vec2<f32>(0.0), vec2<f32>(1.0));
            color += textureSample(material_image, material_sampler, sample_uv);
        }
    }

    return color * normalization_factor;
}
