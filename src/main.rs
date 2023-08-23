use std::{time::SystemTime, thread};

use image::{ImageBuffer, Rgb};
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
    node::Node, to_vector2, visualisation::draw_aabb,
};

use vecto_rs::{Vec2, QuadTree};

const BYTES: &[u8] = include_bytes!("../res/font.ttf");

struct Verlet {
    phys_properties: VerletPhysicsProperties,
    nodes: Vec<Node>,
    font: Font,
    last_run_time: SystemTime,
    mouse_pos: Vec2,
    grabbed_node: usize,
    f : f32,
    auto_fill: bool,
    cam_offset : Vec2,
    do_cam_offset_calc : bool,
    last_mouse_pos : Vec2,
    last_mouse_pos_raw : Vec2,
    mouse_pos_raw: Vec2,
    scale : f32,
    view_tree : bool,
    request_capture : bool
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
            f: 0.0,
            auto_fill : false,
            cam_offset : Vec2::ZERO,
            do_cam_offset_calc : false,
            last_mouse_pos : Vec2::ZERO,
            mouse_pos_raw : Vec2::ZERO,
            last_mouse_pos_raw : Vec2::ZERO,
            scale : 1.0,
            view_tree : false,
            request_capture : false
        }
    }
}

fn main() {
    let win = Window::new_with_options(
        "Verlet",
        WindowCreationOptions::new_windowed(
            speedy2d::window::WindowSize::PhysicalPixels(Vector2::new(720.0 as u32, 720.0 as u32)),
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
    fn on_mouse_wheel_scroll(
            &mut self,
            _helper: &mut speedy2d::window::WindowHelper<()>,
            distance: speedy2d::window::MouseScrollDistance
        ) {
        let y = match distance
        {
            speedy2d::window::MouseScrollDistance::Lines { x: _, y, z: _ } => {y / 100.0},
            speedy2d::window::MouseScrollDistance::Pixels { x: _, y, z: _ } => {y / 100.0},
            speedy2d::window::MouseScrollDistance::Pages { x: _, y, z: _ } => {y / 100.0},
        };
        

        self.scale = (self.scale + y as f32).max(0.01);
    }

    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        graphics.clear_screen(Color::from_hex_rgb(0x0f0e16));

        if self.do_cam_offset_calc
        {
            self.cam_offset = self.cam_offset + (self.mouse_pos_raw - self.last_mouse_pos_raw);
        }

        self.f += 2.0;

        if self.auto_fill
        {
            for _ in 0..1
            {
                let mut n = Node::new(WIDTH / 2.0, HEIGHT / 2.0);
                n.old_pos = n.pos + Vec2(self.f % 0.1, (self.f + 0.1) % 0.1);
                n.colour = ((self.f / 500.0) % 1.0,0.0,(self.f / 1000.0) % 1.0);
                n.radius = (self.f % 10.0) + 10.0;
                self.add_node(n);
            }
        }

        graphics.draw_line(to_vector2(Vec2(0.0,0.0      ) * self.scale + self.cam_offset),to_vector2(Vec2(0.0,HEIGHT  ) * self.scale + self.cam_offset), 1.0, Color::WHITE);
        graphics.draw_line(to_vector2(Vec2(0.0,HEIGHT   ) * self.scale + self.cam_offset),to_vector2(Vec2(WIDTH,HEIGHT) * self.scale + self.cam_offset), 1.0, Color::WHITE);
        graphics.draw_line(to_vector2(Vec2(WIDTH,HEIGHT ) * self.scale + self.cam_offset),to_vector2(Vec2(WIDTH,0.0   ) * self.scale + self.cam_offset), 1.0, Color::WHITE);
        graphics.draw_line(to_vector2(Vec2(WIDTH,0.0    ) * self.scale + self.cam_offset),to_vector2(Vec2(0.0,0.0     ) * self.scale + self.cam_offset), 1.0, Color::WHITE);

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
            node.draw(graphics, self.cam_offset, self.scale);
        }

        if self.phys_properties.collisions_on {
            let tree = Node::collision_check(&mut self.nodes);
            if self.view_tree
            {
                fn walk(tree : &QuadTree<usize>, graphics: &mut speedy2d::Graphics2D, scale_cam : (f32, Vec2))
                {
                    if tree.is_leaf()
                    {
                        draw_aabb(tree.get_bb(), graphics, scale_cam)
                    } else 
                    {
                        let tl = tree.get_tl();
                        walk(&tl, graphics, scale_cam);
    
                        let tr = tree.get_tr();
                        walk(&tr, graphics, scale_cam);
    
                        let bl = tree.get_bl();
                        walk(&bl, graphics, scale_cam);
    
                        let br = tree.get_br();
                        walk(&br, graphics, scale_cam);
                    }
                }
                walk(&tree, graphics, (self.scale, self.cam_offset));
            }
        }

        let now = SystemTime::now();

        let dt = now
            .duration_since(self.last_run_time)
            .unwrap()
            .as_secs_f32();

        let fps: f32 = 1.0 / dt;

        self.last_run_time = now;

        let text = self.font.layout_text(
            format!("FPS: {}\nCircles: {}", fps.floor(), self.nodes.len()).as_str(),
            32.0,
            TextOptions::new(),
        );
        graphics.draw_text((0.0, 0.0), Color::WHITE, &text);


        if self.request_capture
        {
            let data = graphics.capture(speedy2d::image::ImageDataType::RGB);
            thread::spawn(move || {                
                let data = data.data();
    
                let mut image2 = ImageBuffer::new(720, 720);
                
                for x in 0..720
                {
                    for y in 0..720
                    {
                        image2.put_pixel(x, y, Rgb([data[((x * 3 + (y * 720 * 3)) + 0) as usize], data[((x * 3 + (y * 720 * 3)) + 1) as usize], data[((x * 3 + (y * 720 * 3)) + 2) as usize]]));
                    }
                }
    
                image2.save("image.png").unwrap();
            });
            self.request_capture = false;
        }

        self.last_mouse_pos = self.mouse_pos;
        self.last_mouse_pos_raw = self.mouse_pos_raw;

        helper.request_redraw();
    }

    fn on_mouse_move(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        position: speedy2d::dimen::Vec2,
    ) {
        self.mouse_pos = (Vec2(position.x, position.y) - self.cam_offset) / self.scale;
        self.mouse_pos_raw = Vec2(position.x, position.y);
    }

    fn on_mouse_button_up(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        button: MouseButton,
    ) {
        if button == MouseButton::Left {
            if let Some(grabbed) = self.get_mouse_grabbed() {
                grabbed.dont_update = false;
            }
            self.grabbed_node = usize::MAX;
        }

        if button == MouseButton::Middle
        {
            self.do_cam_offset_calc = false;
        }
    }

    fn on_mouse_button_down(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if button == MouseButton::Right {
            if let Some(grabbed) = self.get_mouse_grabbed() {
                grabbed.anchor = !grabbed.anchor;
            }
        }


        if button == MouseButton::Middle
        {
            self.do_cam_offset_calc = true;
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
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(key) = virtual_key_code {
            match key {
                VirtualKeyCode::C => {
                    self.nodes.clear();
                }
                VirtualKeyCode::V => {
                    self.view_tree = !self.view_tree;
                }
                VirtualKeyCode::Backspace =>
                {
                    if self.grabbed_node > self.nodes.len()
                    {
                        return;
                    }
                    self.nodes.remove(self.grabbed_node);
                    self.grabbed_node = usize::MAX;
                }
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::LShift => {
                    self.request_capture = true;
                }
                VirtualKeyCode::Space => {
                    self.auto_fill = !self.auto_fill;
                }
                VirtualKeyCode::N => {
                    self.add_node(Node::new(self.mouse_pos.0, self.mouse_pos.1));
                }
                _ => {}
            }
        }
    }
}
