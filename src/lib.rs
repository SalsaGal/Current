pub mod input;
pub mod math;
pub mod layer;

use input::InputHandler;
use math::Vector2;
use layer::GameLayer;

use sdl2::{
	event::{
		Event,
	},
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

	layer_stack: Vec<Box<dyn GameLayer>>,

	pub input_handler: InputHandler,
	pub running: bool,
}

impl Engine {
	pub fn new(window_title: &str, window_bounds: Vector2<u32>, mut layer: Box<dyn GameLayer>) -> Self {
		let sdl2_context = sdl2::init().unwrap();
		let sdl2_video = sdl2_context.video().unwrap();
		let sdl2_window = WindowBuilder::new(&sdl2_video, window_title, window_bounds.x, window_bounds.y).build().unwrap();

		Self {
			canvas: sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap(),
			event_pump: sdl2_context.event_pump().unwrap(),

			layer_stack: {
				layer.on_push();
				vec![ layer ]
			},

			input_handler: InputHandler::new(),
			running: true
		}
	}

	pub fn update(&mut self) {
		// Update input
		let mut quitting = false;
		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => quitting = true,
				Event::KeyDown { .. } |
				Event::KeyUp { .. } |
				Event::MouseButtonDown { .. } |
				Event::MouseButtonUp { .. } |
				Event::MouseMotion { .. } => self.input_handler.event(event),
				_ => {},
			}
		}
		if quitting {
			self.quit();
			return;
		}

		// Update layers
		let layer_max = self.layer_stack.len() - 1;
		for (index, layer) in self.layer_stack.iter_mut().enumerate() {
			if index == layer_max {
				layer.update_focused();
			} else {
				layer.update_unfocused();
			}
			layer.update();
		}
	}

	pub fn quit(&mut self) {
		for _ in 0 .. self.layer_stack.len() {
			self.pop_layer();
		}
		self.running = false;
	}

	pub fn pop_layer(&mut self) -> Box<dyn GameLayer> {
		self.layer_stack.last_mut().unwrap().on_pop();
		self.layer_stack.pop().unwrap()
	}

	pub fn push_layer(&mut self, mut layer: Box<dyn GameLayer>) {
		layer.on_push();
		self.layer_stack.push(layer);
	}
}