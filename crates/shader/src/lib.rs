#![cfg_attr(target_arch = "spirv", no_std)]

use shared::ShaderConstants;
use spirv_std::glam::*;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

mod sdf;

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let x = ((x - a) / (b - a)).clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

fn col_cart(p: Vec2) -> Vec3 {
    let mut col = Vec3::ZERO;

    let attachment = sdf::disk(p, 0.03);
    let frame = sdf::capsule_x(p, 0.1, 0.02);
    let outer_wheels =
        sdf::disk(p - vec2(0.1, 0.0), 0.04).min(sdf::disk(p - vec2(-0.1, 0.0), 0.04));
    let inner_wheels =
        sdf::disk(p - vec2(0.1, 0.0), 0.015).min(sdf::disk(p - vec2(-0.1, 0.0), 0.015));

    col += smoothstep(0.002, 0.0, frame.max(-outer_wheels.min(attachment)));

    col += smoothstep(0.002, 0.0, attachment.abs());
    col.x += smoothstep(0.0001, 0.0, attachment);
    col += smoothstep(0.002, 0.0, frame.abs().max(-attachment));
    col += smoothstep(0.003, 0.001, outer_wheels.abs());
    col += smoothstep(0.002, 0.0, inner_wheels);
    col
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = (vec2(frag_coord.x, -frag_coord.y)
        - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let cart_pos = vec2(constants.cart_position_x, 0.0);
    let bob_pos = vec2(constants.bob_position_x, constants.bob_position_y);

    let track_col = {
        let d_track = sdf::capsule_x(uv, 0.6, 0.04);
        smoothstep(0.002, 0.0, d_track.abs())
    };
    let bob_col = {
        //let d_bob = sdf::disk(uv - bob_pos, 0.03);
        Vec3::splat(smoothstep(0.002, 0.0, sdf::disk(uv - bob_pos, 0.03).abs()))
            + Vec3::X * smoothstep(0.001, 0.0, sdf::disk(uv - bob_pos, 0.029))
    };

    let rod_col = {
        let d_rod = sdf::capsule(uv, cart_pos, bob_pos, 0.005);
        smoothstep(0.002, 0.0, d_rod)
    };

    let mut col = Vec3::ZERO;
    col += track_col;
    col += col_cart(uv - cart_pos);
    col += rod_col;
    if bob_col.x > bob_col.y {
        col = bob_col;
    } else {
        col += bob_col;
    }

    *output = col.powf(2.2).extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}
