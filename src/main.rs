use std::time::{SystemTime, Duration};

use speedy2d::{
    color::Color,
    dimen::Vector2,
    window::{WindowCreationOptions, WindowHandler, VirtualKeyCode},
    Window, font::{Font, TextLayout, TextOptions},
};
use verlet_multithreaded::{
    consts::{HEIGHT, WIDTH},
    Node::Node,
    physics::VerletPhysicsProperties, maths::Vec2,
};

const BYTES : &[u8] = include_bytes!("../res/font.ttf");

struct Verlet {
    phys_properties: VerletPhysicsProperties,
    nodes: Vec<Node>,
    font : Font,
    last_run_time: SystemTime,
}

impl Default for Verlet
{
    fn default() -> Self {
        Self { phys_properties: Default::default(), nodes: Default::default(), font: Font::new(BYTES).unwrap(), last_run_time : SystemTime::now() }
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

    for i in 0..1000 {
        let x = WIDTH / 1000.0;
        verlet.nodes.push(Node::new(i as f32 * x, i as f32 * x))
    }

    verlet.phys_properties.floor_height = HEIGHT;

    win.run_loop(verlet);
}

impl WindowHandler for Verlet {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        graphics.clear_screen(Color::from_hex_rgb(0x0f0e16));

        for x in ((WIDTH as i32 / 10)..WIDTH as i32).step_by(WIDTH as usize / 10) {
            graphics.draw_line((x as f32, 0.0), (x as f32, HEIGHT), 1.0, Color::WHITE)
        }

        for y in ((HEIGHT as i32 / 10)..HEIGHT as i32).step_by(HEIGHT as usize / 10) {
            graphics.draw_line((0.0, y as f32), (WIDTH, y as f32), 1.0, Color::WHITE)
        }

        for node in &mut self.nodes {
            node.update(&self.phys_properties);
            node.constrain(Vec2::ZERO, Vec2(WIDTH, HEIGHT));
            node.draw(graphics);
        }

        let now = SystemTime::now();

        let dt = now.duration_since(self.last_run_time).unwrap().as_secs_f32();

        let fps : f32 = 1.0 / dt;

        self.last_run_time = now;

        let text = self.font.layout_text(format!("FPS: {}", fps.floor()).as_str(), 32.0, TextOptions::new());
        graphics.draw_text((0.0, 0.0), Color::WHITE, &text);

        helper.request_redraw();

    }

    fn on_key_down(
            &mut self,
            helper: &mut speedy2d::window::WindowHelper<()>,
            virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
            scancode: speedy2d::window::KeyScancode
        ) {
        if let Some(key) = virtual_key_code
        {
            match key
            {
                VirtualKeyCode::Escape =>
                {
                    helper.terminate_loop()
                }
                _ => {}
            }
        }
    }
}
