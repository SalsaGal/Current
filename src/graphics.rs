pub use sdl2::pixels::Color;

use crate::math::{Rect, Vector2};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
	pub background_color: Color,
	sprite_cache: HashMap<String, Texture>,
	text_cache: HashMap<Text, Texture>,
	ttf: Sdl2TtfContext,
}

impl GraphicsHandler {
	pub fn new(canvas: WindowCanvas) -> Self {
		Self {
			canvas,
			background_color: Color::BLACK,
			sprite_cache: HashMap::new(),
			text_cache: HashMap::new(),
			ttf: sdl2::ttf::init().unwrap(),
		}
	}

	pub fn get_bounds(texture: &Texture, pos: Vector2<i32>) -> Rect {
		Rect::new(pos.x, pos.y, texture.query().width, texture.query().height)
	}

	pub fn get_texture(&mut self, image: &mut Image) -> &mut Texture {
		self.sprite_cache.get_mut(image.render()).unwrap()
	}

	pub fn get_text_texture(&mut self, text: &Text) -> &mut Texture {
		self.text_cache.get_mut(text).unwrap()
	}

	pub fn render(&mut self, image: &mut Image, pos: Vector2<i32>) {
		let path = image.render();
		if !self.sprite_cache.contains_key(path) {
			self.sprite_cache.insert(path.to_owned(), self.canvas.texture_creator().load_texture(path.clone()).unwrap());
		}

		let texture = self.sprite_cache.get(path).unwrap();
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

pub struct Animation {
	frames: Vec<String>,
	frame: usize,
	frame_length: Duration,
	last_frame: Instant,
}

impl Animation {
	pub fn new(frames: Vec<String>, frame_length: Duration) -> Self {
		Self {
			frames,
			frame: 0,
			frame_length,
			last_frame: Instant::now(),
		}
	}

	pub fn render(&mut self) -> &str {
		if Instant::now().duration_since(self.last_frame) >= self.frame_length {
			self.frame = (self.frame + 1) % self.frames.len();
			self.last_frame = Instant::now();
		}

		self.frames.get(self.frame).unwrap()
	}
}

pub enum Image {
	Animation(Animation),
	None,
	Sprite(String),
}

impl Image {
	pub fn render(&mut self) -> &str {
		match self {
			Image::Animation(animation) => animation.render(),
			Image::None => "",
			Image::Sprite(path) => path,
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