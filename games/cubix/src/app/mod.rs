use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::renderer::Renderer;

pub fn run() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("cubix")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut renderer = unsafe { Renderer::create(&window)? };
    event_loop.run(move |event, elwt| match event {
        Event::AboutToWait => window.request_redraw(),
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::RedrawRequested if !elwt.exiting() => {
                unsafe { renderer.render(&window) }.unwrap()
            }
            WindowEvent::CloseRequested => {
                elwt.exit();
                unsafe {
                    renderer.destroy();
                }
            }
            _ => {}
        },
        _ => {}
    })?;

    Ok(())
}
