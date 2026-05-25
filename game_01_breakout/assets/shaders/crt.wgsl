#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct CrtSettings { distortion: f32, scanline_intensity: f32, time: f32 }
@group(0) @binding(2) var<uniform> settings: CrtSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    // barrel distortion: push uv outward from center by distance²
    let center = uv - 0.5;
    uv = uv + center * dot(center, center) * settings.distortion;

    // chromatic split — sample R/G/B at slightly different offsets
    let r = textureSample(screen_texture, texture_sampler, uv + vec2(0.002, 0.0)).r;
    let g = textureSample(screen_texture, texture_sampler, uv).g;
    let b = textureSample(screen_texture, texture_sampler, uv - vec2(0.002, 0.0)).b;

    // scanlines: darken every other row, animated by time
    let scan = sin(uv.y * 800.0 + settings.time * 10.0) * 0.5 + 0.5;
    let line = 1.0 - settings.scanline_intensity * scan;

    return vec4(vec3(r, g, b) * line, 1.0);
}
