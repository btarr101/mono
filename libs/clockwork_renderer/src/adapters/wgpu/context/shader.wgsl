struct CameraUniform {
    view_projection: mat4x4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct InstanceInput {
    @location(2) transform_0: vec4<f32>,
    @location(3) transform_1: vec4<f32>,
    @location(4) transform_2: vec4<f32>,
    @location(5) transform_3: vec4<f32>,
    @location(6) color: vec4<f32>,
    @location(7) uv_window: vec4<f32>,
    // @location(8) flip_bits: u32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;

@vertex
fn vertex_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let instance_transform = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3
    );

    out.clip_position = camera_uniform.view_projection * instance_transform * vec4<f32>(model.position, 1.0);
    out.color = instance.color;

    // let hflip = (instance.flip_bits & 0x1u) != 0u; // bit 0
    // let vflip = (instance.flip_bits & 0x2u) != 0u; // bit 1

    var uv = instance.uv_window.xy + (model.texture_coordinates * instance.uv_window.zw);
    // uv.x = select(uv.x, instance.uv_window.x + instance.uv_window.z - (model.texture_coordinates.x * instance.uv_window.z), hflip);
    // uv.y = select(uv.y, instance.uv_window.y + instance.uv_window.w - (model.texture_coordinates.y * instance.uv_window.w), vflip);

    out.texture_coordinates = uv;

    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = in.color * textureSample(diffuse_texture, diffuse_sampler, in.texture_coordinates);

    if color.a < 0.01 {
        discard;
    }

    return color;
}
