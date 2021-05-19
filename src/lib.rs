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
use std::ops::{Index, IndexMut};

/// An enum used to store different types of data accessible across the whole game
#[derive(Debug)]
pub enum Data {
	Any(Box<dyn Any>),
	Bool(bool),
	String(String),
}

/// A struct that stores data that should be accessible by all layers.
pub struct GlobalData<'data> {
	data: HashMap<&'data str, Data>,
}

impl<'data> GlobalData<'data> {
	fn new() -> Self {
		Self {
			data: HashMap::new(),
		}
	}

	pub fn any_ref(&self, var: &str) -> Option<&Box<dyn Any>> {
		if let Some(Data::Any(to_ret)) = self.data.get(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn any_mut(&mut self, var: &str) -> Option<&mut Box<dyn Any>> {
		if let Some(Data::Any(to_ret)) = self.data.get_mut(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn bool_ref(&self, var: &str) -> Option<&bool> {
		if let Some(Data::Bool(to_ret)) = self.data.get(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn bool_mut(&mut self, var: &str) -> Option<&mut bool> {
		if let Some(Data::Bool(to_ret)) = self.data.get_mut(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn string_ref(&self, var: &str) -> Option<&String> {
		if let Some(Data::String(to_ret)) = self.data.get(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn string_mut(&mut self, var: &str) -> Option<&mut String> {
		if let Some(Data::String(to_ret)) = self.data.get_mut(var) {
			Some(to_ret)
		} else {
			None
		}
	}

	pub fn get(&self, var: &str) -> Option<&Data> {
		self.data.get(var)
	}

	pub fn get_mut(&mut self, var: &str) -> Option<&mut Data> {
		self.data.get_mut(var)
	}

	pub fn set(&mut self, var: &'data str, data: Data) {
		self.data.insert(var, data);
	}
}

impl Index<&str> for GlobalData<'_> {
	type Output = Data;
	fn index(&self, index: &str) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl IndexMut<&str> for GlobalData<'_> {
	fn index_mut(&mut self, index: &str) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

/// The `Engine` struct stores all components of the engine and manages them all.
pub struct Engine<'engine> {
	event_pump: EventPump,
	pub audio: AudioHandler,
	pub graphics: GraphicsHandler,

	layer_stack: Vec<Box<dyn GameLayer>>,

	pub data: GlobalData<'engine>,
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

		let mut data = GlobalData::new();

		Self {
			event_pump: sdl2_context.event_pump().unwrap(),
			audio: AudioHandler::new(),
			graphics: GraphicsHandler::new(sdl2_window.into_canvas().accelerated().present_vsync().build().unwrap()),

			layer_stack: {
				layer.on_push(&mut data);
				vec![ layer ]
			},

			data,
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
			transition = layer.update(&mut self.data, &mut self.audio, &mut self.graphics, &self.input_handler, index == layer_max);
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
		self.layer_stack.last_mut().unwrap().on_pop(&mut self.data);
		let stack_size = self.layer_stack.len();
		if stack_size > 1 {
			// If this is none then we are popping the last layer, in which case nothing gains focus
			if let Some(layer_gaining_focus) = self.layer_stack.get_mut(stack_size - 2) {
				layer_gaining_focus.on_gain_focus(&mut self.data);
			}
		} else {
			// If there is nothing to gain focus then we can just quit the program
			self.running = false;
		}
		self.layer_stack.pop().unwrap()
	}

	pub fn push_layer(&mut self, mut layer: Box<dyn GameLayer>) {
		layer.on_push(&mut self.data);
		self.layer_stack.last_mut().unwrap().on_lose_focus(&mut self.data);
		self.layer_stack.push(layer);
	}
}
