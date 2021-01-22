pub use sdl2::rect::Rect;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vector2<T> {
	pub fn new(x: T, y: T) -> Self where T: Copy {
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

	pub fn point_collides(&self, rect: Rect) -> bool where T: Into<i32> + Copy {
		self.x.into() > rect.x &&
		self.x.into() < rect.x + rect.w &&
		self.y.into() > rect.y &&
		self.y.into() < rect.y + rect.h
	}
}

impl<T> Add<Vector2<T>> for Vector2<T> where T: Add<T, Output = T> {
	type Output = Vector2<T>;

	fn add(self, other: Vector2<T>) -> Self::Output {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl<T> Sub<Vector2<T>> for Vector2<T> where T: Sub<T, Output = T> {
	type Output = Vector2<T>;

	fn sub(self, other: Vector2<T>) -> Self::Output {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl<T> Mul<Vector2<T>> for Vector2<T> where T: Mul<T, Output = T> {
	type Output = Vector2<T>;

	fn mul(self, other: Vector2<T>) -> Self::Output {
		Vector2 {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl<T> Div<Vector2<T>> for Vector2<T> where T: Div<T, Output = T> {
	type Output = Vector2<T>;

	fn div(self, other: Vector2<T>) -> Self::Output {
		Vector2 {
			x: self.x / other.x,
			y: self.y / other.y,
		}
	}
}