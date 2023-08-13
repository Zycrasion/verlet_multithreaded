use speedy2d::{Graphics2D, color::Color};
use crate::{maths::Vec2, physics::VerletPhysicsProperties};

pub struct Node
{
    pub pos : Vec2,
    pub old_pos : Vec2,
    pub radius : f32
}

impl Node
{
    pub fn new(x : f32, y : f32) -> Self
    {
        Self
        {
            pos : Vec2(x, y),
            old_pos : Vec2(x, y),
            radius : 5.0
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D)
    {
        graphics.draw_circle(self.pos, 5.0, Color::WHITE)
    }

    pub fn update(&mut self, phys : &VerletPhysicsProperties)
    {
        let vel = self.pos - self.old_pos;
        self.old_pos = self.pos;

        self.pos = self.pos + phys.gravity + vel;
    }

    pub fn constrain(&mut self, min : Vec2, max : Vec2)
    {
        self.pos = self.pos.clamp(min + self.radius , max - self.radius);
    }
}