pub mod graphics;
pub mod input;
pub mod math;
pub mod layer;

use graphics::GraphicsHandler;
use input::InputHandler;
use math::Vector2;
use layer::{GameLayer, Transition};

use sdl2::{
	event::Event,
	EventPump,
	video::WindowBuilder,
};

use std::any::Any;
use std::collections::HashMap;

pub type GlobalData<'data> = HashMap<&'data str, Box<dyn Any>>;

pub struct Engine<'engine> {
	event_pump: EventPump,
	pub graphics: GraphicsHandler,

	layer_stack: Vec<Box<dyn GameLayer>>,

	pub global_data: GlobalData<'engine>,
	pub input_handler: InputHandler,
	pub running: bool,
}

impl<'engine> Engine<'_> {
	pub fn new(window_title: &str, window_bounds: Vector2<u32>, mut layer: Box<dyn GameLayer>) -> Self {
		let sdl2_context = sdl2::init().unwrap();
		let sdl2_video = sdl2_context.video().unwrap();
		let sdl2_window = WindowBuilder::new(&sdl2_video, window_title, window_bounds.x, window_bounds.y).build().unwrap();

		Self {
			event_pump: sdl2_context.event_pump().unwrap(),
			graphics: GraphicsHandler::new(sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap()),

			layer_stack: {
				layer.on_push(&mut GlobalData::new());
				vec![ layer ]
			},

			global_data: GlobalData::new(),
			input_handler: InputHandler::new(),
			running: true
		}
	}

	pub fn update(&mut self) {
		// Rnedering preparations
		self.graphics.canvas.set_draw_color(self.graphics.background_color);
		self.graphics.canvas.clear();

		// Update input
		self.input_handler.update();
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
		let mut transition = Transition::None;
		for (index, layer) in self.layer_stack.iter_mut().enumerate() {
			transition = layer.update(&mut self.global_data, &mut self.graphics, &self.input_handler, index == layer_max);
		}
		match transition {
			Transition::None => {}
			Transition::Pop => { self.pop_layer(); },
			Transition::PopMulti(depth) => {
				for _ in 0 .. depth {
					self.pop_layer();
				}
			}
			Transition::Push(layer) => self.push_layer(layer),
			Transition::PushMulti(to_add) => {
				for layer in to_add.into_iter() {
					self.push_layer(layer);
				}
			}
			Transition::Quit => self.quit(),
		}

		// Render
		self.graphics.canvas.present();
	}

	pub fn quit(&mut self) {
		for _ in 0 .. self.layer_stack.len() - 1 {
			self.pop_layer();
		}
		self.running = false;
	}

	pub fn pop_layer(&mut self) -> Box<dyn GameLayer> {
		self.layer_stack.last_mut().unwrap().on_pop(&mut self.global_data);
		let stack_size = self.layer_stack.len();
		if let Some(layer_gaining_focus) = self.layer_stack.get_mut(stack_size - 2) {
			// If this is none then we are popping the last layer, in which case nothing gains focus
			layer_gaining_focus.on_gain_focus(&mut self.global_data);
		} else {
			// If there is nothing to gain focus then we can just quit the program
			self.running = false;
		}
		self.layer_stack.pop().unwrap()
	}

	pub fn push_layer(&mut self, mut layer: Box<dyn GameLayer>) {
		layer.on_push(&mut self.global_data);
		self.layer_stack.last_mut().unwrap().on_lose_focus(&mut self.global_data);
		self.layer_stack.push(layer);
	}
}