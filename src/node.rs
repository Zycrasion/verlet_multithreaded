

use crate::{physics::VerletPhysicsProperties, to_vector2};
use speedy2d::{color::Color, Graphics2D};
use vecto_rs::{Vec2, QuadTree};
use crate::consts::{HEIGHT, WIDTH};

#[derive(Copy, Clone, PartialEq)]
pub struct Node {
    pub pos: Vec2,
    pub old_pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub dont_update: bool,
    pub anchor: bool,
    pub colour: (f32, f32, f32)
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
            colour : (1.0,1.0,1.0)
        }
    }

    pub fn repel(&mut self, pos: Vec2) {
        if self.anchor || self.dont_update {
            return;
        }

        self.pos = pos - (self.pos - pos).normalized() / pos.dist(&self.pos);
    }

    pub fn collision_check(nodes: &mut Vec<Node>) -> QuadTree<usize>
    {
        let mut tree = QuadTree::new(0.0,0.0, WIDTH, HEIGHT, 50, 20.0, 1000);
        for i in 0..nodes.len()
        {
            tree.add(i, nodes[i].pos);
        }

        for i1 in 0..nodes.len() {
            for i2 in tree.query(nodes[i1].pos) {
                if nodes[i1] == nodes[i2.1] {
                    continue;
                }

                let dist = nodes[i1].pos.dist(&nodes[i2.1].pos);
                if dist <= nodes[i1].radius + nodes[i2.1].radius
                {
                    let n = (nodes[i2.1].pos - nodes[i1].pos).normalized();

                    let dist_from_n1 = dist * (nodes[i1].radius / (nodes[i1].radius + nodes[i2.1].radius));

                    let c = nodes[i1].pos + n * dist_from_n1;

                    let n1_r = nodes[i1].radius;
                    nodes[i1].update_pos_no_vel(c - n * n1_r);
                    let n2_r = nodes[i2.1].radius;
                    nodes[i2.1].update_pos_no_vel(c + n * n2_r);
                }

                if dist == 0.0
                {
                    nodes[i1].update_pos(Vec2(100.0, 100.0));
                    nodes[i2.1].update_pos(Vec2(200.0, 200.0));
                }
            }
        }
        tree
    }

    pub fn draw(&self, graphics: &mut Graphics2D, cam_offset : Vec2, scale: f32) {

        let _stroke_colour = if self.anchor
        {
            Color::from_hex_rgb(0x1d3658)
        } else
        {
            Color::GRAY
        };

        let _fill_colour = if self.anchor
        {
            Color::from_hex_rgb(0x447a9c)
        } else 
        {
            Color::WHITE
        };

        let fill_colour = Color::from_rgb(self.colour.0, self.colour.1.max(0.3), self.colour.2.max(0.3));

        // graphics.draw_circle(to_vector2(self.pos), self.radius, stroke_colour);
        graphics.draw_circle(to_vector2((self.pos * scale) + cam_offset), self.radius * scale, fill_colour)
    }

    pub fn update_pos(&mut self, pos: Vec2) {
        self.old_pos = self.pos;
        self.pos = pos;
    }

    pub fn update_pos_no_vel(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn update(&mut self, phys: &VerletPhysicsProperties) {
        if !self.pos.0.is_normal() || !self.pos.0.is_normal() 
        {
            self.dont_update = true;
        }

        if self.dont_update || self.anchor {
            return;
        }

        let mut vel = self.pos - self.old_pos;
        self.old_pos = self.pos;

        vel = vel * phys.friction;

        self.pos = self.pos + phys.gravity + vel;
    }

    pub fn constrain(&mut self, min: Vec2, max: Vec2) {
        self.pos = self.pos.clamp(min + self.radius, max - self.radius);

        if self.pos.0 + self.radius > max.0
        {
            self.old_pos = self.old_pos - Vec2(max.0, 0.0);
            self.pos =  self.pos - Vec2(max.0, 0.0);
        }

        if self.pos.1 + self.radius > max.1
        {
            self.old_pos = self.old_pos - Vec2(0.0, max.1);
            self.pos =  self.pos - Vec2(0.0, max.1);
        }

        if self.pos.0 - self.radius < min.0
        {
            self.old_pos = Vec2(max.0, 0.0) + self.old_pos;
            self.pos =  Vec2(max.0, 0.0) + self.pos;
        }

        if self.pos.1 - self.radius < min.1
        {
            self.old_pos = Vec2(0.0, max.1) + self.old_pos;
            self.pos =  Vec2(0.0, max.1) + self.pos;
        }
    }
}
