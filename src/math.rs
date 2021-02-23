//! Structs and functions for general maths.

pub use sdl2::rect::Rect;

use std::ops::{Add, Div, Mul, Sub};

/// A struct which contains two values of the same type, typically used to represent
/// 2D values, like map positions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vector2<T> {
	/// Creates a Vector2 with two seperate values
	pub fn new(x: T, y: T) -> Self where T: Copy {
		Self {
			x,
			y,
		}
	}

	/// Convenience function to create a Vector2 where the two values are the same. 
	pub fn square(w: T) -> Self where T: Copy {
		Self {
			x: w,
			y: w,
		}
	}

	/// Convenience function to create a Vector2 where the two values are 0. 
	pub fn origin() -> Self where T: Default {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}

	/// Converts the Vector2 into a tuple, typically for debugging.
	pub fn as_tuple(&self) -> (T, T) where T: Copy {
		( self.x, self.y )
	}

	/// Check if the point is in the bounds of an SDL2 `Rect`.
	pub fn point_collides(&self, rect: Rect) -> bool where T: Into<i32> + Copy {
		self.x.into() >= rect.x &&
		self.x.into() <= rect.x + rect.w &&
		self.y.into() >= rect.y &&
		self.y.into() <= rect.y + rect.h
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

/// Checks if two SDL2 `Rect`s overlap
pub fn rect_collides(a: Rect, b: Rect) -> bool {
	let x_hitbox = a.w as f32 / 2.0 + b.w as f32 / 2.0;
	let y_hitbox = a.h as f32 / 2.0 + b.h as f32 / 2.0;

	let a_center = Vector2::new(a.w as f32 / 2.0 + a.x as f32, a.h as f32 / 2.0 + a.y as f32);
	let b_center = Vector2::new(b.w as f32 / 2.0 + b.x as f32, b.h as f32 / 2.0 + b.y as f32);

	(a_center.x - b_center.x).abs() < x_hitbox && (a_center.y - b_center.y).abs() < y_hitbox
}