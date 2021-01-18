mod math;

use sdl2::{
	EventPump,
	render::{
		WindowCanvas,
	},
	video::{
		WindowBuilder,
	},
};

pub struct Engine {
	canvas: WindowCanvas,
	event_pump: EventPump,
}

impl Engine {
	pub fn new() -> Self {
		let sdl2_context = sdl2::init().unwrap();
		let sdl2_video = sdl2_context.video().unwrap();
		let sdl2_window = WindowBuilder::new(&sdl2_video, "Current", 640, 480).build().unwrap();

		Self {
			canvas: sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap(),
			event_pump: sdl2_context.event_pump().unwrap(),
		}
	}

	pub fn update(&mut self) {
		
	}
}