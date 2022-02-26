#![windows_subsystem = "windows"]

use softbuffer::GraphicsContext;
use std::env;
use std::path::Path;
use std::{thread, time};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
mod frames;

fn draw(width: u32, height: u32) -> Vec<u32> {
	let mut buffer = frames::new_frame(width,height);
	let white = frames::Color(255,255,255);
	let green = frames::Color(0,255,0);
	buffer.fill(white);
	//buffer.set_box(100,100,250,200,green);
	buffer.draw_circle(100,100,100,green);
    return buffer.pixels_buffer;
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(640, 480));
    let users_monitor = &window.current_monitor().unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();
    let monitor_height = users_monitor.size().height;
    let monitor_width = users_monitor.size().width;
    let event_loop_proxy = event_loop.create_proxy();


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                let (width, height) = {
                    let size = graphics_context.window().inner_size();
                    (size.width, size.height)
                };
                let buffer = draw(width, height);
                graphics_context.set_buffer(
                    &buffer,
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                )
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    })
}
