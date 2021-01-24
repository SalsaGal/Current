use crate::math::{Rect, Vector2};

use sdl2::event::Event;

use std::collections::HashMap;

pub use sdl2::{
	keyboard::Scancode,
	mouse::MouseButton,
};

pub struct InputHandler {
	keys_down: HashMap<Scancode, bool>,
	keys_pressed: HashMap<Scancode, bool>,
	keys_released: HashMap<Scancode, bool>,

	buttons_down: HashMap<MouseButton, bool>,
	buttons_pressed: HashMap<MouseButton, bool>,
	buttons_released: HashMap<MouseButton, bool>,

	pub click_pos: HashMap<MouseButton, Vector2<i32>>,
	pub mouse_pos: Vector2<i32>,
}

impl InputHandler {
	pub fn new() -> Self {
		Self {
			keys_down: HashMap::new(),
			keys_pressed: HashMap::new(),
			keys_released: HashMap::new(),

			buttons_down: HashMap::new(),
			buttons_pressed: HashMap::new(),
			buttons_released: HashMap::new(),

			click_pos: HashMap::new(),
			mouse_pos: Vector2::origin(),
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
				self.keys_released.insert(scancode.unwrap(), true);
			},
			Event::MouseButtonDown { mouse_btn, x, y, .. } => {
				self.buttons_down.insert(mouse_btn, true);
				self.buttons_pressed.insert(mouse_btn, true);
				if !self.click_pos.contains_key(&mouse_btn) {
					self.click_pos.insert(mouse_btn, Vector2::new(x, y));
				}
			}
			Event::MouseButtonUp { mouse_btn, .. } => {
				self.buttons_down.insert(mouse_btn, false);
				self.buttons_released.insert(mouse_btn, true);
				self.click_pos.remove(&mouse_btn);
			}
			Event::MouseMotion { x, y, .. } => {
				self.mouse_pos = Vector2::new(x, y);
			}
			_ => unreachable!()			// Other functions will never be passed
		}
	}

	pub fn update(&mut self) {
		self.keys_pressed.clear();
		self.keys_released.clear();
		self.buttons_pressed.clear();
		self.buttons_released.clear();
	}

	pub fn key_is(&self, key: Scancode, state: InputState) -> bool {
		match state {
			InputState::Down => *self.keys_down.get(&key).unwrap_or(&false),
			InputState::Pressed => *self.keys_pressed.get(&key).unwrap_or(&false),
			InputState::Up => !self.keys_down.get(&key).unwrap_or(&false),
		}
	}

	pub fn button_is(&self, button: MouseButton, state: InputState) -> bool {
		match state {
			InputState::Down => *self.buttons_down.get(&button).unwrap_or(&false),
			InputState::Pressed => *self.buttons_pressed.get(&button).unwrap_or(&false),
			InputState::Up => !self.buttons_down.get(&button).unwrap_or(&false),
		}
	}

	pub fn clicked_in_bounds(&self, button: MouseButton, bounds: Rect) -> bool {
		self.button_is(button, InputState::Pressed) && self.mouse_pos.point_collides(bounds)
	}
}

pub enum InputState {
	Down,
	Pressed,
	Released,
	Up,
}