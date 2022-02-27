#[derive(Clone, Copy)] 
pub struct Color(pub u32,pub u32,pub u32);

pub struct Frame {
	width: u32,
	height: u32,
	pub pixels_buffer: Vec<u32>
}

impl Frame {
	pub fn get_position(&self, x:u32, y:u32) -> usize {
		(y*self.width+x) as usize

	}
	pub fn set_pixel(&mut self,x:u32,y:u32,pixel:Color) -> () {
		if x > self.width || y > self.height || x < 0 || y < 0 {
			panic!("Wrong values for x,y given: {:?}, and maximum {:?}",(x,y),(self.width,self.height) );
		}
		let position = self.get_position(x,y);
		self.pixels_buffer[position] = pixel.to_int()

	}
	pub fn fill(&mut self, pixel:Color) -> () {
		for index in (0..self.width*self.height) {
			self.pixels_buffer[index as usize] = pixel.to_int()
		}
	}
	pub fn set_box(&mut self, left:u32,top:u32,right:u32, bottom:u32,pixel:Color) -> () {
		for new_x in (left..right) {
			for new_y in (top..bottom) {
				// println!("{}, {}",x+x_offset,y+y_offset );
				self.set_pixel(new_x,new_y,pixel)

			} 
		}
	}
	pub fn draw_line(&mut self, x0:u32,y0:u32,x1:u32,y1:u32,pixel:Color) -> (){
	    let dx = x1 as i32 - x0 as i32;
	    let dy = y1 as i32 - y0 as i32;
	    let mut D = 2*dy as i32 - dx as i32;
	    let mut y = y0;

	    for x in (x0..x1) {
	        self.set_pixel(x,y,pixel);
	        if D > 0 {
	            y += 1;
	            D -= 2*dx as i32 }
	        D +=  2*dy as i32
		}
	}
	pub fn draw_8_points(&mut self, xc:u32, yc:u32,x:u32,y:u32,pixel:Color) -> () {


		self.set_pixel(xc+x, yc+y, pixel);
	    self.set_pixel(xc-x, yc+y, pixel);
	    self.set_pixel(xc+x, yc-y, pixel);
	    self.set_pixel(xc-x, yc-y, pixel);
	    self.set_pixel(xc+y, yc+x, pixel);
	    self.set_pixel(xc-y, yc+x, pixel);
	    self.set_pixel(xc+y, yc-x, pixel);
	    self.set_pixel(xc-y, yc-x, pixel);

	}
	pub fn draw_circle(&mut self, x_center:u32, y_center:u32,radius:u32,pixel:Color) -> () {
		let mut x = 0;
		let mut y = radius;
		self.draw_8_points(x_center, y_center, x, y,pixel);
		let mut d = 3 - 2 * radius as i32; 
		loop {

			if !(y>= x) {
				break;
			}
			x += 1; 
		if (d > 0)
        {
            y -= 1;
            //println!("{:?} {} {}", 4 * (x as i32 - y as i32 ) + 10 );
            d +=  4 * (x as i32 - y as i32 ) + 10;
        }  else {
            d += (4 * x) as i32 + 6 as i32; }
            self.draw_8_points(x_center, y_center, x, y,pixel);
		}




	}
}

impl Color {
	pub fn to_int(self) -> u32 {
		self.0 *256*256 + self.1 *256 + self.2 

	}
}

pub fn new_frame(width: u32, height: u32) -> Frame {
	let mut vec = Vec::with_capacity((width*height).try_into().unwrap());
	vec.resize((width*height) as usize, 0);
	Frame {width: width, height: height, pixels_buffer: vec}
}