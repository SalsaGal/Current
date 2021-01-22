pub use sdl2::rect::Rect;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vector2<T> where T: Clone + Copy {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn square(w: T) -> Self {
		Self {
			x: w,
			y: w,
		}
	}

	pub fn origin() -> Self where T: Default {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}

	pub fn point_collides(&self, rect: Rect) -> bool where T: Into<i32> {
		self.x.into() > rect.x &&
		self.x.into() < rect.x + rect.w &&
		self.y.into() > rect.y &&
		self.y.into() < rect.y + rect.h
	}
}