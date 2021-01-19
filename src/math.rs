pub use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn square(w: T) -> Self where T: Copy {
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
}

pub fn point_collides(pos: Vector2<i32>, rect: Rect) -> bool {
	pos.x > rect.x &&
	pos.x < rect.x + rect.w &&
	pos.y > rect.y &&
	pos.y < rect.y + rect.h
}