pub mod math;

use math::Vector2;

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
	pub fn new(window_title: &str, window_bounds: Vector2<u32>) -> Self {
		let sdl2_context = sdl2::init().unwrap();
		let sdl2_video = sdl2_context.video().unwrap();
		let sdl2_window = WindowBuilder::new(&sdl2_video, window_title, window_bounds.x, window_bounds.y).build().unwrap();

		Self {
			canvas: sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap(),
			event_pump: sdl2_context.event_pump().unwrap(),
		}
	}

	pub fn update(&mut self) {
	}
}