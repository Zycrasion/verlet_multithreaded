use std::{sync::Arc, rc::Rc, ops::Deref, cell::RefCell};

use crate::{physics::VerletPhysicsProperties, to_vector2};
use speedy2d::{color::Color, Graphics2D};
use vecto_rs::{Vec2, QuadTree};

#[derive(Copy, Clone, PartialEq)]
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

        self.pos = pos - (self.pos - pos).normalized() / pos.dist(&self.pos);
    }

    pub fn collision_check(nodes: &mut Vec<Arc<RefCell<Node>>>, mut tree: QuadTree<Arc<RefCell<Node>>>) ->  QuadTree<Arc<RefCell<Node>>> {
        for i1 in 0..nodes.len() {
            for i2 in tree.query(nodes[i1].borrow().pos) {
                if nodes[i1].deref() == i2.1.deref() {
                    continue;
                }

                let dist = nodes[i1].borrow().pos.dist(&i2.1.borrow().pos);
                if dist <= nodes[i1].borrow().radius + i2.1.borrow().radius
                {
                    let n = (i2.1.borrow().pos - nodes[i1].borrow().pos).normalized();

                    let dist_from_n1 = dist * (nodes[i1].borrow().radius / (nodes[i1].borrow().radius + i2.1.borrow().radius));

                    let c = nodes[i1].borrow().pos + n * dist_from_n1;

                    let n1_r = nodes[i1].borrow().radius;
                    nodes[i1].borrow_mut().update_pos_no_vel(c - n * n1_r);
                    let n2_r = i2.1.borrow().radius;
                    i2.1.borrow_mut().update_pos_no_vel(c + n * n2_r);
                }

                if dist == 0.0
                {
                    nodes[i1].borrow_mut().update_pos(Vec2(100.0, 100.0));
                    i2.1.borrow_mut().update_pos(Vec2(200.0, 200.0));
                }
            }
        }

        tree
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

        graphics.draw_circle(to_vector2(self.pos), self.radius, stroke_colour);
        graphics.draw_circle(to_vector2(self.pos), self.radius - 5.0, fill_colour)
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
