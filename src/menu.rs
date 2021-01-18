use crate::graphics::{GraphicsHandler, Image};
use crate::input::InputHandler;
use crate::layer::GameLayer;
use crate::math::Vector2;
use crate::Transition;

pub struct Menu {
	labels: Vec<Label>,
}

impl Menu {
	pub fn new(labels: Vec<Label>) -> Self {
		Self {
			labels,
		}
	}
}

impl GameLayer for Menu {
	fn update(&mut self, graphics: &mut GraphicsHandler, _input: &InputHandler, _focused: bool) -> Transition {
		for label in self.labels.iter() {
			graphics.render(&label.image, label.pos);
		}

		Transition::None
	}
}

pub struct Label {
	pub pos: Vector2<i32>,
	pub image: Image,
}