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

	pub fn render(&mut self, image: Image) {
		let path = image.render();
		if !self.sprite_cache.contains_key(&path) {
			self.sprite_cache.insert(path.clone(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}

		self.canvas.copy(self.sprite_cache.get(&path).unwrap(), None, None).unwrap();
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