#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

type Point = (f32, f32);

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Cubix")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut app = unsafe { App::create(&window)? };

    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::RedrawRequested if !elwt.exiting() => {
                    unsafe { app.render(&window) }.unwrap();
                }
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe { app.destroy() };
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}

fn fill_scanline(poly: &[Point]) {
    let min_y = poly.iter().map(|p| p.1).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = poly.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    for y in min_y..=max_y {
        let scan_y = y as f32 + 0.5;
        let mut xs = Vec::new();

        for i in 0..poly.len() {
            let (x1,y1) = poly[i];
            let (x2,y2) = poly[(i + 1) % poly.len()];

            // Ignore horizontal edge
            if y1 == y2 {
                continue;
            }

            // Count inters. once per vertex

            let intersects = (y1 <= scan_y && scan_y < y2) || (y2 <= scan_y && scan_y < y1);

            if intersects {
                let x = x1 + (scan_y - y1) * (x2 - x1) / (y2 - y1);
                xs.push(x);
            }
        }

        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in xs.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0].ceil() as i32;
                let x_end = pair[1].floor() as i32;

                for x in x_start..=x_end {
                    // PAINT pixel at (x, y)

                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct App {}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {}
}

#[derive(Clone, Debug, Default)]
struct AppData {}
