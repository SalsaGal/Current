use sdl2::event::Event;

use std::collections::HashMap;

pub use sdl2::{
	keyboard::Scancode,
};

pub struct InputHandler {
	keys_down: HashMap<Scancode, bool>,
	keys_pressed: HashMap<Scancode, bool>,
}

impl InputHandler {
	pub fn new() -> Self {
		Self {
			keys_down: HashMap::new(),
			keys_pressed: HashMap::new(),
		}
	}
	
	pub fn event(&mut self, event: Event) {
		match event {
			Event::KeyDown { scancode, .. } => {
				self.keys_down.insert(scancode.unwrap(), true);
				self.keys_pressed.insert(scancode.unwrap(), true);
			},
			Event::KeyUp { scancode, .. } => {
				self.keys_down.insert(scancode.unwrap(), false);
			},
			_ => {}//unreachable!()			// Other functions will never be passed
		}
	}

	pub fn update(&mut self) {
		self.keys_pressed.clear();
	}

	pub fn key_is(&self, key: Scancode, state: InputState) -> bool {
		match state {
			InputState::Down => *self.keys_down.get(&key).unwrap_or(&false),
			InputState::Pressed => *self.keys_pressed.get(&key).unwrap_or(&false),
			InputState::Up => !self.keys_down.get(&key).unwrap_or(&false),
		}
	}
}

pub enum InputState {
	Down,
	Pressed,
	Up,
}