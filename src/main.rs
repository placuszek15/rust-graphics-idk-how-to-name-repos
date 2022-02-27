#![windows_subsystem = "windows"]

use softbuffer::GraphicsContext;
use std::env;
use std::path::Path;
use std::cmp;
use std::{thread, time};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
mod frames;

fn get_points(x_center:u32,y_center:u32,radius:u32,n:u32) -> Vec<Vec<u32>> {
	let mut vec = Vec::new();
	for angle in (0..360).step_by(360/n as usize){
		let radians = (angle as f32).to_radians();
		let index_sin = radians.sin();
		let index_cos = radians.cos();
		vec.push(vec![
			(x_center as f32+
			(radius as f32)*index_cos) as u32,
			(y_center as f32+
			(radius as f32)*index_sin) as u32])
	}
	return vec 




}

fn draw(width: u32, height: u32) -> Vec<u32> {
	let mut buffer = frames::new_frame(width,height);
	let white = frames::Color(255,255,255);
	let green = frames::Color(0,255,0);
	let red = frames::Color(255,0,0);
	buffer.fill(white);
	//buffer.set_box(100,100,250,200,green);
	buffer.draw_circle(320,240,239,green);
	let circle_points = get_points(320,240,239,20);
	for i in circle_points {
		println!("{:?}",i);
		if i[0] > 320 && i[1] > 239 {
			buffer.draw_line(
				320,239,
				i[0],i[1]
				,red)
		}
		else {
			buffer.draw_line(
				i[0],i[1],
				320,239
				,red)
		}
	}
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
