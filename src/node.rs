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
            radius: 5.0,
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
        for i1 in 0..nodes.len() {
            for i2 in 0..nodes.len() {
                if i1 == i2 {
                    continue;
                }

                let dist = nodes[i1].pos.dist(&nodes[i2].pos);
                if dist < nodes[i1].radius.max(nodes[i2].radius) * 2.0
                {
                    let m_total = nodes[i1].mass + nodes[i2].mass;
                    let mut m1 = nodes[i1].mass / m_total;
                    let mut m2 = nodes[i2].mass / m_total;

                    if nodes[i1].anchor || nodes[i1].dont_update { m2 = 0.0; m1 = 1.0; }
                    if nodes[i2].anchor || nodes[i2].dont_update { m1 = 0.0; m2 = 1.0; }

                    let dist_needed = nodes[i1].pos - nodes[i2].pos;

                    let n1 = nodes[i1].pos;
                    nodes[i1].update_pos(n1 + dist_needed * m2);
                    let n2 = nodes[i2].pos;
                    nodes[i2].update_pos(n2 - dist_needed * m1);
                }
            }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle(self.pos, self.radius, Color::GRAY);
        graphics.draw_circle(self.pos, self.radius - 2.0, Color::WHITE)
    }

    pub fn update_pos(&mut self, pos: Vec2) {
        self.old_pos = self.pos;
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
