use std::{iter::{zip, Zip}, sync::{Arc, Mutex, MutexGuard}, thread, time::{Duration, Instant}};

use speedy2d::{color::Color, dimen::Vector2, window::{self, WindowHandler}, Window};
use vecto_rs::{QuadTree, Vec2};

const CIRCLE_SIZE : f32 = 3.;

pub struct Simulation
{
    pub circles_old : Vec<Vec2>,
    pub render_circles : Arc<Mutex<Vec<Vec2>>>,
    pub sim_steps : u32,
    pub instant : Instant,
    pub window_size : Arc<Mutex<(f32, f32)>>
}

fn main()
{
    let simulation = Simulation::new();

    let app_window = AppWindow{render_circles:simulation.render_circles.clone(), frames : 0, instant : Instant::now(), window_size : simulation.window_size.clone()};
    simulation.begin();
    let win_size = app_window.window_size.lock().unwrap().clone();
    Window::new_centered("Hi", (win_size.0 as u32, win_size.1 as u32)).unwrap().run_loop(app_window)
}

impl Simulation
{
    pub fn new() -> Self
    {
        let mut circles_old = vec![];
        let mut render_circles = vec![];
        for i in 0..10_000
        {
            let row = (i % 100);
            render_circles.push(Vec2(i as f32 * CIRCLE_SIZE * 1.1, (row as f32 * CIRCLE_SIZE * 1.1)));
            circles_old.push(Vec2(i as f32 * CIRCLE_SIZE * 1.1 - 0.1, (row as f32 * CIRCLE_SIZE * 1.1)));
        }
        Self
        {
            circles_old,
            render_circles : Arc::new(Mutex::new(render_circles)),
            sim_steps : 0,
            instant : Instant::now(),
            window_size : Arc::new(Mutex::new((720., 720.)))
        }
    }

    pub fn update(&mut self)
    {
        if self.instant.elapsed().as_secs_f32() > 1.0
        {
            println!("TPS: {}", self.sim_steps);
            self.instant = Instant::now();
            self.sim_steps = 0;
        }
        self.sim_steps += 1;

        let circle_lock = self.render_circles.lock().unwrap();
        let mut circles = circle_lock.clone();
        drop(circle_lock);
        let win_size = self.window_size.lock().unwrap().clone();
        let mut quad_tree : QuadTree<usize> = QuadTree::new(0., 0., win_size.0, win_size.1, 200, CIRCLE_SIZE * 2., 12);

        for (i, (circle, circle_old)) in zip::<&mut Vec<Vec2>, &mut Vec<Vec2>>(circles.as_mut(), &mut self.circles_old).enumerate()
        {
            quad_tree.add(i, *circle);

            let mut velocity = *circle - *circle_old;
            velocity = velocity * 0.97;
            velocity.1 += 0.009;
            *circle_old = *circle;
            *circle = *circle + velocity;
            *circle = circle.clamp(Vec2(0., 0.), Vec2(win_size.0, win_size.1));
        }
        
        for i1 in 0..circles.len()
        {
            for v in quad_tree.query(circles[i1])
            {
                let i2 = v.1;
                if i1 == i2
                {
                    continue;
                }
                let dist = circles[i1].dist(&circles[i2]).max(0.01);
                if dist < CIRCLE_SIZE * 2.
                {
                    let midpoint = (circles[i1] + circles[i2]) / 2.;
                    
                    let new_pos_i1 = midpoint + (circles[i1] - circles[i2]) * CIRCLE_SIZE / dist;
                    let new_pos_i2 = midpoint + (circles[i2] - circles[i1]) * CIRCLE_SIZE / dist;

                    // let new_old_pos_i1 = self.circles_old[i1] + (circles[i1] - new_pos_i1);
                    // let new_old_pos_i2 = self.circles_old[i2] + (circles[i2] - new_pos_i2);

                    circles[i1] = new_pos_i1; circles[i2] = new_pos_i2;

                    // self.circles_old[i1] = new_old_pos_i1;
                    // self.circles_old[i2] = new_old_pos_i2;
                }
            }
        }


        *self.render_circles.lock().unwrap().as_mut() = circles;

        // thread::sleep(Duration::from_secs_f64(1. / 100.));
    }

    pub fn begin(mut self)
    {
        thread::spawn(move ||{loop{self.update()}});
    }
}

struct AppWindow
{
    pub render_circles : Arc<Mutex<Vec<Vec2>>>,
    pub frames : u32,
    pub instant : Instant,
    pub window_size : Arc<Mutex<(f32, f32)>>
}

impl WindowHandler for AppWindow
{
    fn on_resize(&mut self, helper: &mut speedy2d::window::WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2)
    {
        *self.window_size.lock().unwrap() = (size_pixels.into_f32().x, size_pixels.into_f32().y);
    }

    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D
    )
    {
        self.frames += 1;
        if self.instant.elapsed().as_secs_f32() > 1.0
        {
            println!("FPS: {}", self.frames);
            self.instant = Instant::now();
            self.frames = 0;
        }
        graphics.clear_screen(Color::BLACK);
        let circle_lock = self.render_circles.lock().unwrap();
        let circles = circle_lock.clone();
        drop(circle_lock);
        let mut r  = 0;
        let mut g = 0;
        let mut b = 0;
        for circle in circles
        {
            graphics.draw_circle(Vector2::new(circle.0, circle.1), CIRCLE_SIZE, Color::from_int_rgb(r.max(50), g.max(50), b.max(50)));
            r = r.overflowing_add(1).0;
            if r == 255
            {
                g = g.overflowing_add(1).0;
            }

            if g == 255
            {
                b = b.overflowing_add(1).0;
            }
        }

        helper.request_redraw();
    }
}