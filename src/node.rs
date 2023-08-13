use speedy2d::{Graphics2D, color::Color};
use crate::{maths::Vec2, physics::VerletPhysicsProperties};

#[derive(Copy, Clone)]
pub struct Node
{
    pub pos : Vec2,
    pub old_pos : Vec2,
    pub radius : f32,
    pub dont_update : bool,
    pub anchor : bool
}

impl Node
{
    pub fn new(x : f32, y : f32) -> Self
    {
        Self
        {
            pos : Vec2(x, y),
            old_pos : Vec2(x, y),
            radius : 5.0,
            dont_update : false,
            anchor : false
        }
    }

    pub fn repel(&mut self, pos : Vec2)
    {
        if self.anchor || self.dont_update
        {
            return;
        }

        self.pos = pos + ((self.pos - pos).normalized());
    }

    pub fn draw(&self, graphics: &mut Graphics2D)
    {
        graphics.draw_circle(self.pos, self.radius, Color::GRAY);
        graphics.draw_circle(self.pos, self.radius - 2.0, Color::WHITE)
    }

    pub fn update_pos(&mut self, pos : Vec2)
    {
        self.old_pos = self.pos;
        self.pos = pos;
    }

    pub fn update(&mut self, phys : &VerletPhysicsProperties)
    {
        if self.dont_update || self.anchor
        {
            return;
        }
        
        let vel = self.pos - self.old_pos;
        self.old_pos = self.pos;

        self.pos = self.pos + phys.gravity + vel;
    }

    pub fn constrain(&mut self, min : Vec2, max : Vec2)
    {
        self.pos = self.pos.clamp(min + self.radius , max - self.radius);
    }
}