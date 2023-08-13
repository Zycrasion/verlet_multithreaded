use crate::{maths::Vec2, physics::VerletPhysicsProperties};
use speedy2d::{color::Color, Graphics2D};

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: Vec2,
    pub old_pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub dont_update: bool,
    pub anchor: bool,
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2(x, y),
            old_pos: Vec2(x, y),
            radius: 20.0,
            dont_update: false,
            anchor: false,
            mass : 1.0,
        }
    }

    pub fn repel(&mut self, pos: Vec2) {
        if self.anchor || self.dont_update {
            return;
        }

        self.pos = pos + ((self.pos - pos).normalized());
    }

    pub fn collision_check(nodes: &mut Vec<Node>) {
        for _ in 0..100
        {
            for i1 in 0..nodes.len() {
                for i2 in 0..nodes.len() {
                    if i1 == i2 {
                        continue;
                    }
    
                    let dist = nodes[i1].pos.dist(&nodes[i2].pos);
                    if dist < nodes[i1].radius + nodes[i2].radius
                    {
                        let m_total = nodes[i1].mass + nodes[i2].mass;
                        let mut m = nodes[i1].mass / m_total;
    
                        if nodes[i1].anchor || nodes[i1].dont_update { m = 0.0; }
                        if nodes[i2].anchor || nodes[i2].dont_update { m = 1.0; }
                        let mut dist_needed = (nodes[i1].pos - nodes[i2].pos).normalized();
                        if (nodes[i2].anchor || nodes[i2].dont_update) && (nodes[i1].anchor || nodes[i1].dont_update) {dist_needed = Vec2::ZERO}

                        let n1 = nodes[i1].pos;
                        nodes[i1].update_pos_no_vel(n1 + (dist_needed * m));
                        let n2 = nodes[i2].pos;
                        nodes[i2].update_pos_no_vel(n2 - (dist_needed * (1.0 - m)));
                    }

                    if dist == 0.0
                    {
                        nodes[i1].update_pos(Vec2(100.0, 100.0));
                        nodes[i2].update_pos(Vec2(200.0, 200.0));
                    }
                }
            }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {

        let stroke_colour = if self.anchor
        {
            Color::from_hex_rgb(0x1d3658)
        } else
        {
            Color::GRAY
        };

        let fill_colour = if self.anchor
        {
            Color::from_hex_rgb(0x447a9c)
        } else 
        {
            Color::WHITE
        };

        graphics.draw_circle(self.pos, self.radius, stroke_colour);
        graphics.draw_circle(self.pos, self.radius - 2.0, fill_colour)
    }

    pub fn update_pos(&mut self, pos: Vec2) {
        self.old_pos = self.pos;
        self.pos = pos;
    }

    pub fn update_pos_no_vel(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn update(&mut self, phys: &VerletPhysicsProperties) {
        if self.dont_update || self.anchor {
            return;
        }

        let mut vel = self.pos - self.old_pos;
        self.old_pos = self.pos;

        if self.pos.1 + self.radius >= phys.floor_height
        {
            vel = vel * phys.ground_friction;
        }

        vel = vel * phys.friction;

        self.pos = self.pos + phys.gravity + vel;
    }

    pub fn constrain(&mut self, min: Vec2, max: Vec2) {
        self.pos = self.pos.clamp(min + self.radius, max - self.radius);
    }
}
