use speedy2d::{Graphics2D, color::Color};
use vecto_rs::{AABB, Vec2};

use crate::to_vector2;

pub fn draw_aabb(aabb : AABB, graphics : &mut Graphics2D, scale_cam : (f32, Vec2))
{
    let v1 = aabb.start;
    let v2 = aabb.start + Vec2(aabb.size.0, 0.0);
    let v4 = aabb.start + Vec2(0.0, aabb.size.1);
    let v3 = aabb.start + aabb.size;

    graphics.draw_line(to_vector2(v1 * scale_cam.0 + scale_cam.1), to_vector2(v2 * scale_cam.0 + scale_cam.1), 1.0, Color::GREEN);
    graphics.draw_line(to_vector2(v2 * scale_cam.0 + scale_cam.1), to_vector2(v3 * scale_cam.0 + scale_cam.1), 1.0, Color::GREEN);
    graphics.draw_line(to_vector2(v3 * scale_cam.0 + scale_cam.1), to_vector2(v4 * scale_cam.0 + scale_cam.1), 1.0, Color::GREEN);
    graphics.draw_line(to_vector2(v4 * scale_cam.0 + scale_cam.1), to_vector2(v1 * scale_cam.0 + scale_cam.1), 1.0, Color::GREEN);
}