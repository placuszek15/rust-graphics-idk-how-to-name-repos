#![windows_subsystem = "windows"]

use softbuffer::GraphicsContext;
use std::env;
use std::path::Path;
use std::cmp;
use std::{thread, time};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent,VirtualKeyCode,KeyboardInput,ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
mod frames;

fn get_points(x_center:u32,y_center:u32,radius:u32,n:u32) -> Vec<Vec<u32>> {
	let mut vec = Vec::new();
	let mut theta = 0.0;
	let delta_theta = 360.0/n as f32; 
	loop{
		if theta > 360.0 {
			break
		}
		let radians = (270.0+theta as f32).to_radians();
		let index_sin = radians.sin();
		let index_cos = radians.cos();
		vec.push(vec![
			(x_center as f32+
			(radius as f32)*index_cos) as u32,
			(y_center as f32+
			(radius as f32)*index_sin) as u32]);
		theta += delta_theta;
	}
	return vec 
}


fn digital_sum(num: u32,base:u32) -> u32 {
	//println!("calcing the sum of {}",num );
	let mut x = num; 	
	let mut total = 0;
	loop {
		if x <= 0 {
			break;
		}
		//println!("t:{:?} x:{} xmb:{}",total,x,x%base );
		total += x %base; 
		x = x/base 
	}
	//println!("sum of {} is {}",num,total );
	return total
}

fn digital_root(num:u32, base:u32) -> u32 {
	let mut x = num;
	loop {
		if x < base {
			break;
		}
		x = digital_sum(x,base)
	}
	return x
}



fn draw(width: u32, height: u32,amount:u32, mult:u32) -> Vec<u32> {
	let mut buffer = frames::new_frame(width,height);
	let white = frames::Color(255,255,255);
	let green = frames::Color(0,255,0);
	let red = frames::Color(255,0,0);
	buffer.fill(white);
	//buffer.set_box(100,100,250,200,green);
	buffer.draw_circle(320,240,239,green);
	let circle_points = get_points(320,240,239,amount);
	let mut points = (1..amount).collect::<Vec<u32>>();
	let mut current = 1;
	loop {
		if points.len() == 0 {
			break
		}
		let mut current = points[0];
		//println!("new loop at {}",current );
		loop {
			//println!("{:?} {}",current,points.iter().any(|e| e == &current) );
		 	if !points.iter().any(|e| e == &current){
		 		break;
		 	}
			points.retain(|element| element != &current);
			let old = current;
			current = digital_root(current*mult,amount);
			//println!("new current is: {:?}",current );
			let ratio = (255.0*(current as f64/amount as f64)) as u32;
			
			buffer.draw_line(
				circle_points[current as usize][0],circle_points[current as usize][1],
				circle_points[old as usize][0],circle_points[old as usize][1]
				,frames::Color(ratio,255 - ratio,127)
			);
	}
	}
	
		
    return buffer.pixels_buffer;
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(640, 480));
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();
    let mut amount = 10;
    let mut mult = 3;

    event_loop.run(move |event, _, control_flow| 
    {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                let (width, height) = {
                    let size = graphics_context.window().inner_size();
                    (size.width, size.height)
                };
                let buffer = draw(width, height,amount,mult);
                graphics_context.set_buffer(
                    &buffer,
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                )
            }

            Event::WindowEvent {
                event,
                ..
            } => match event {
            	WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
            	WindowEvent::KeyboardInput {
            		input: KeyboardInput {
            			virtual_keycode: Some(real_key),
            			state: ElementState::Pressed,
            			..
            		}, 
            		.. 
            	} => match real_key {
            		VirtualKeyCode::Up => {
            			amount += 1;
            			println!("amount is now {}",amount );
            		    graphics_context.window().request_redraw();}
            		VirtualKeyCode::Down => {
            			amount -= 1;
            			println!("amount is now {}",amount );
            		    graphics_context.window().request_redraw();}
            		VirtualKeyCode::Right => {
            			mult += 1;
            			println!("multiplier is now {}",mult );
            		    graphics_context.window().request_redraw();}
            		VirtualKeyCode::Left => {
            			mult -= 1;
            			println!("multiplier is now {}",mult );
            		    graphics_context.window().request_redraw();}
            		_ => ()
            	},
            	_ => ()
            },

            _ => (),
        }
    }
    )
}
