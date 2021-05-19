//! Structs and functions for general maths.

pub use sdl2::rect::Rect;

use std::default::Default;
use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, MulAssign, SubAssign};

/// A struct which contains two values of the same type, typically used to represent
/// 2D values, like map positions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
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
		Self::default()
	}

	/// Returns a Vector2 which converts the types it stores.
	pub fn from<U>(other: Vector2<U>) -> Self where T: From<U> {
		Self {
			x: other.x.into(),
			y: other.y.into(),
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

impl<T> Default for Vector2<T> where T: Default {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
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

impl<T> AddAssign<Vector2<T>> for Vector2<T> where T: AddAssign<T> {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
	}
}

impl<T> SubAssign<Vector2<T>> for Vector2<T> where T: SubAssign<T> {
	fn sub_assign(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
	}
}

impl<T> MulAssign<Vector2<T>> for Vector2<T> where T: MulAssign<T> {
	fn mul_assign(&mut self, other: Self) {
		self.x *= other.x;
		self.y *= other.y;
	}
}

impl<T> DivAssign<Vector2<T>> for Vector2<T> where T: DivAssign<T> {
	fn div_assign(&mut self, other: Self) {
		self.x /= other.x;
		self.y /= other.y;
	}
}
