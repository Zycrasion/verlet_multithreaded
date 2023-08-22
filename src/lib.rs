use speedy2d::dimen::Vector2;
use vecto_rs::Vec2;

pub mod consts;
pub mod physics;
pub mod node;
pub mod visualisation;

pub fn to_vector2(a : Vec2) -> Vector2<f32>
{
    Vector2 { x: a.x(), y: a.y() }
}