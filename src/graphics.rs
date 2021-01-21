pub use sdl2::pixels::Color;

use crate::math::{Rect, Vector2};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;

use std::collections::HashMap;

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
	sprite_cache: HashMap<String, Texture>,
	text_cache: HashMap<Text, Texture>,
	ttf: Sdl2TtfContext,
}

impl GraphicsHandler {
	pub fn new(canvas: WindowCanvas) -> Self {
		Self {
			canvas,
			sprite_cache: HashMap::new(),
			text_cache: HashMap::new(),
			ttf: sdl2::ttf::init().unwrap(),
		}
	}

	pub fn get_bounds(texture: &Texture, pos: Vector2<i32>) -> Rect {
		Rect::new(pos.x, pos.y, texture.query().width, texture.query().height)
	}

	pub fn get_texture(&mut self, image: &Image, pos: Vector2<i32>) -> &mut Texture {
		self.sprite_cache.get_mut(&image.render()).unwrap()
	}

	pub fn get_text_texture(&mut self, text: &Text, pos: Vector2<i32>) -> &mut Texture {
		self.text_cache.get_mut(text).unwrap()
	}

	pub fn render(&mut self, image: &Image, pos: Vector2<i32>) {
		let path = image.render();
		if !self.sprite_cache.contains_key(&path) {
			self.sprite_cache.insert(path.clone(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}

		let texture = self.sprite_cache.get(&path).unwrap();
		self.canvas.copy(texture, None, Self::get_bounds(texture, pos)).unwrap();
	}

	pub fn render_text(&mut self, text: &Text, pos: Vector2<i32>) {
		if !self.text_cache.contains_key(text) {
			let font = self.ttf.load_font(text.font_path.clone(), text.size).unwrap();
			self.text_cache.insert(
				text.clone(),
				self.canvas.create_texture_from_surface(font.render(&text.text.to_string()).blended_wrapped(text.color, self.canvas.output_size().unwrap().0).unwrap()).unwrap()
			);
		}

		let texture = self.text_cache.get(text).unwrap();
		self.canvas.copy(texture, None, Self::get_bounds(texture, pos)).unwrap();
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

#[derive(Eq, Hash, PartialEq)]
pub struct Text {
    pub text: String,
    pub font_path: String,
    pub size: u16,
    pub color: Color,
}

impl Clone for Text {
	fn clone(&self) -> Self {
		Self {
			text: self.text.clone(),
			font_path: self.font_path.clone(),
			size: self.size,
			color: self.color,
		}
	}
}