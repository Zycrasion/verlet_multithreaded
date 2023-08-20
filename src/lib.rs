use speedy2d::dimen::Vector2;
use vecto_rs::Vec2;

pub mod consts;
pub mod physics;
#[allow(non_snake_case)]
pub mod Node; // I have no clue why rust recognises this as a valid module name

pub fn to_vector2(a : Vec2) -> Vector2<f32>
{
    Vector2 { x: a.x(), y: a.y() }
}