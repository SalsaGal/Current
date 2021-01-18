use crate::math::{Rect, Vector2};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};

use std::collections::HashMap;

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
	sprite_cache: HashMap<String, Texture>,
}

impl GraphicsHandler {
	pub fn new(canvas: WindowCanvas) -> Self {
		Self {
			canvas,
			sprite_cache: HashMap::new(),
		}
	}

	pub fn get_bounds(&mut self, image: &Image, pos: Vector2<i32>) -> Rect {
		let path = image.render();
		if !self.sprite_cache.contains_key(&path) {
			self.sprite_cache.insert(path.clone(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}
		let texture = self.sprite_cache.get(&path).unwrap();
		Rect::new(pos.x, pos.y, texture.query().width, texture.query().height)
	}

	pub fn render(&mut self, image: &Image, pos: Vector2<i32>) {
		let path = image.render();
		if !self.sprite_cache.contains_key(&path) {
			self.sprite_cache.insert(path.clone(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}

		let bounds = self.get_bounds(image, pos);
		self.canvas.copy(self.sprite_cache.get(&path).unwrap(), None, bounds).unwrap();
	}
}

pub enum Image {
	None,
	Sprite(String),
}

impl Image {
	pub fn render(&self) -> String {
		match self {
			Image::None => "".to_owned(),
			Image::Sprite(path) => path.to_owned(),
		}
	}
}