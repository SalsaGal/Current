//! The Current engine is a game engine with the main goals of being extremely
//! simple to implement into a game.
//! 
//! Very little is needed to initialise the engine:
//! ```
//! use current::layer::EmptyLayer;
//! use current::prelude::*;
//! 
//! fn main() {
//! 	let mut engine = Engine::new("Title", Vector2::new(640, 480), Box::new(EmptyLayer));
//! 
//! 	while engine.running {
//! 		engine.update();
//! 
//! 		// This example is capped to 60 FPS
//! 		std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
//! 	}
//! }
//! ```

pub mod audio;
pub mod graphics;
pub mod input;
pub mod math;
pub mod layer;
pub mod prelude;

use audio::AudioHandler;
use graphics::GraphicsHandler;
use input::InputHandler;
use math::Vector2;
use layer::{GameLayer, Transition};

use sdl2::{
	EventPump,
	event::Event,
	video::WindowBuilder,
};

use std::any::Any;
use std::collections::HashMap;

/// A struct that stores data that should be accessible by all layers.
pub struct GlobalData<'data> {
	data: HashMap<&'data str, Box<dyn Any>>,
}

impl<'data> GlobalData<'data> {
	fn new() -> Self {
		Self {
			data: HashMap::new(),
		}
	}

	pub fn get<Type: 'static>(&mut self, var: &str) -> Option<&Type> {
		let data = self.data.get(var);

		if let Some(val) = data {
			Some(val.downcast_ref::<Type>().unwrap())
		} else {
			None
		}
	}

	pub fn get_mut<Type: 'static>(&mut self, var: &str) -> Option<&mut Type> {
		let data = self.data.get_mut(var);

		if let Some(val) = data {
			Some(val.downcast_mut::<Type>().unwrap())
		} else {
			None
		}
	}

	pub fn remove(&mut self, var: &'data str) -> Option<Box<dyn Any>> {
		self.data.remove(var)
	}

	pub fn set(&mut self, var: &'data str, val: Box<dyn Any>) {
		self.data.insert(var, val);
	}
}

/// The `Engine` struct stores all components of the engine and manages them all.
pub struct Engine<'engine> {
	event_pump: EventPump,
	pub audio: AudioHandler,
	pub graphics: GraphicsHandler,

	layer_stack: Vec<Box<dyn GameLayer>>,

	pub global_data: GlobalData<'engine>,
	pub input_handler: InputHandler,
	pub running: bool,

	pub frame_limit: Option<u8>,
}

impl<'engine> Engine<'_> {
	/// Create a new instance of the Engine and all its components.
	/// 
	/// If `window_bounds` is `Vector2::origin()` then the window will be created full screen.
	pub fn new(window_title: &str, window_bounds: Vector2<u32>, mut layer: Box<dyn GameLayer>) -> Self {
		let sdl2_context = sdl2::init().unwrap();
		let sdl2_video = sdl2_context.video().unwrap();
		let sdl2_window = if window_bounds == Vector2::origin() {
			WindowBuilder::new(&sdl2_video, window_title, window_bounds.x, window_bounds.y).fullscreen_desktop().build().unwrap()
		} else {
			WindowBuilder::new(&sdl2_video, window_title, window_bounds.x, window_bounds.y).build().unwrap()
		};

		let mut global_data = GlobalData::new();

		Self {
			event_pump: sdl2_context.event_pump().unwrap(),
			audio: AudioHandler::new(),
			graphics: GraphicsHandler::new(sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap()),

			layer_stack: {
				layer.on_push(&mut global_data);
				vec![ layer ]
			},

			global_data,
			input_handler: InputHandler::new(),
			running: true,

			frame_limit: None,
		}
	}

	/// Updates all components of the engine.
	/// 
	/// This function should be called every frame.
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
			transition = layer.update(&mut self.global_data, &mut self.audio, &mut self.graphics, &self.input_handler, index == layer_max);
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
			Transition::Replace(stack) => {
				while self.layer_stack.len() > 0 {
					self.pop_layer();
				}
				self.running = true;	// pop_layer calls quit when it realises the stack is empty which it is temporarily
				for layer in stack.into_iter() {
					self.push_layer(layer);
				}
			}
		}

		// Render
		self.graphics.canvas.present();

		// Frame limit
		if let Some(frame_rate) = self.frame_limit {
			std::thread::sleep(std::time::Duration::from_millis(1000 / frame_rate as u64));
		}
	}

	pub fn quit(&mut self) {
		while self.layer_stack.len() > 0 {
			self.pop_layer();
		}
	}

	pub fn pop_layer(&mut self) -> Box<dyn GameLayer> {
		self.layer_stack.last_mut().unwrap().on_pop(&mut self.global_data);
		let stack_size = self.layer_stack.len();
		if stack_size > 1 {
			// If this is none then we are popping the last layer, in which case nothing gains focus
			if let Some(layer_gaining_focus) = self.layer_stack.get_mut(stack_size - 2) {
				layer_gaining_focus.on_gain_focus(&mut self.global_data);
			}
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