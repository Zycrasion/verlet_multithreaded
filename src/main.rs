use std::{
    sync::{mpsc::channel, Arc, Mutex},
    time::{Duration, SystemTime}, cell::RefCell, borrow::BorrowMut, ops::Deref,
};

use speedy2d::{
    color::Color,
    dimen::Vector2,
    font::{Font, TextLayout, TextOptions},
    window::{MouseButton, VirtualKeyCode, WindowCreationOptions, WindowHandler},
    Window,
};
use verlet_multithreaded::{
    consts::{HEIGHT, WIDTH},
    physics::VerletPhysicsProperties,
    node::Node,
};

use vecto_rs::{Vec2, QuadTree};

use rayon::{prelude::*, ThreadPoolBuilder};

const BYTES: &[u8] = include_bytes!("../res/font.ttf");

struct Verlet {
    phys_properties: VerletPhysicsProperties,
    nodes: Vec<Node>,
    font: Font,
    last_run_time: SystemTime,
    mouse_pos: Vec2,
    grabbed_node: usize,
    f : f32,
}

impl Default for Verlet {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            phys_properties: Default::default(),
            nodes: Default::default(),
            font: Font::new(BYTES).unwrap(),
            last_run_time: SystemTime::now(),
            grabbed_node: usize::MAX,
            f: 0.0
        }
    }
}

fn main() {
    let win = Window::new_with_options(
        "Verlet",
        WindowCreationOptions::new_windowed(
            speedy2d::window::WindowSize::PhysicalPixels(Vector2::new(WIDTH as u32, HEIGHT as u32)),
            None,
        )
        .with_resizable(false)
        .with_decorations(false)
        .with_multisampling(16)
        .with_vsync(true),
    )
    .unwrap();

    let mut verlet = Verlet::default();

    let nodes = 0;

    for i in 0..nodes {
        let x = WIDTH / nodes as f32;
        verlet.add_node(Node::new(i as f32 * x, i as f32 * x))
    }

    verlet.phys_properties.floor_height = HEIGHT;

    win.run_loop(verlet);
}

impl Verlet {
    pub fn get_mouse_grabbed(&mut self) -> Option<&mut Node> {
        if self.grabbed_node < self.nodes.len() {
            Some(&mut self.nodes[self.grabbed_node])
        } else {
            None
        }
    }

    pub fn add_node(&mut self, node: Node)
    {
        let a1 = node;
        self.nodes.push(a1);
    }
}

impl WindowHandler for Verlet {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        graphics.clear_screen(Color::from_hex_rgb(0x0f0e16));

        self.f += 2.0;

        let mut n = Node::new(1.0,1.0);
        n.old_pos = n.pos - 4.0;
        n.colour = ((self.f / 500.0) % 1.0,0.0,(self.f / 1000.0) % 1.0);
        self.add_node(n);

        let mut n = Node::new(20.0,1.0);
        n.old_pos = n.pos - 4.0;
        n.colour = (((self.f) / 500.0) % 1.0,0.0,(self.f / 1000.0) % 1.0);
        self.add_node(n);

        let mut n = Node::new(40.0,1.0);
        n.old_pos = n.pos - 4.0;
        n.colour = ((self.f / 500.0) % 1.0,0.0,(self.f / 1000.0) % 1.0);
        self.add_node(n);

        for x in ((WIDTH as i32 / 10)..WIDTH as i32).step_by(WIDTH as usize / 10) {
            graphics.draw_line((x as f32, 0.0), (x as f32, HEIGHT), 1.0, Color::WHITE)
        }

        for y in ((HEIGHT as i32 / 10)..HEIGHT as i32).step_by(HEIGHT as usize / 10) {
            graphics.draw_line((0.0, y as f32), (WIDTH, y as f32), 1.0, Color::WHITE)
        }

        let pos = self.mouse_pos;

        if let Some(grabbed) = self.get_mouse_grabbed() {
            grabbed.update_pos(pos);
        }

        for node in &mut self.nodes
        {
            node.update(&self.phys_properties);
            node.constrain(Vec2::ZERO, Vec2(WIDTH, HEIGHT));
        }

        for node in &self.nodes {
            node.draw(graphics);
        }

        if self.phys_properties.collisions_on {
            Node::collision_check(&mut self.nodes);
        }

        let now = SystemTime::now();

        let dt = now
            .duration_since(self.last_run_time)
            .unwrap()
            .as_secs_f32();

        let fps: f32 = 1.0 / dt;

        self.last_run_time = now;

        let text = self.font.layout_text(
            format!("FPS: {}", fps.floor()).as_str(),
            32.0,
            TextOptions::new(),
        );
        graphics.draw_text((0.0, 0.0), Color::WHITE, &text);

        helper.request_redraw();
    }

    fn on_mouse_move(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        position: speedy2d::dimen::Vec2,
    ) {
        self.mouse_pos = Vec2(position.x, position.y);
    }

    fn on_mouse_button_up(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        button: MouseButton,
    ) {
        if button == MouseButton::Left {
            if let Some(grabbed) = self.get_mouse_grabbed() {
                grabbed.dont_update = false;
            }
            self.grabbed_node = usize::MAX;
        }
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if button == MouseButton::Right {
            if let Some(grabbed) = self.get_mouse_grabbed() {
                grabbed.anchor = !grabbed.anchor;
            }
        }

        if button == MouseButton::Left {
            let mut lowest_dist = f32::MAX;
            let mut closest_node: usize = usize::MAX;

            let mut index = 0;

            for node in &self.nodes {
                let dist = node.pos.dist(&self.mouse_pos);
                if dist <= node.radius && dist <= lowest_dist {
                    lowest_dist = dist;
                    closest_node = index;
                }
                index += 1;
            }

            self.grabbed_node = closest_node;
            if let Some(grabbed) = self.get_mouse_grabbed() {
                grabbed.dont_update = true;
            }
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(key) = virtual_key_code {
            match key {
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::LShift => {

                }
                VirtualKeyCode::N => {
                    self.add_node(Node::new(self.mouse_pos.0, self.mouse_pos.1));
                }
                _ => {}
            }
        }
    }
}
