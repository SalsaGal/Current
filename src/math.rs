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

pub fn rect_collides(a: Rect, b: Rect) -> bool {
	let x_hitbox = a.w as f32 / 2.0 + b.w as f32 / 2.0;
	let y_hitbox = a.h as f32 / 2.0 + b.h as f32 / 2.0;

	let a_center = Vector2::new(a.w as f32 / 2.0 + a.x as f32, a.h as f32 / 2.0 + a.y as f32);
	let b_center = Vector2::new(b.w as f32 / 2.0 + b.x as f32, b.h as f32 / 2.0 + b.y as f32);

	(a_center.x - b_center.x).abs() < x_hitbox && (a_center.y - b_center.y).abs() < y_hitbox
}

pub fn rect_intersection_depth(a: Rect, b: Rect) -> Option<Vector2<f32>> {
	let a_half_width = a.w as f32 / 2.0;
	let a_half_height = a.h as f32 / 2.0;
	let b_half_width = b.w as f32 / 2.0;
	let b_half_height = b.h as f32 / 2.0;

	let a_center = Vector2::new(a.x as f32 + a_half_width, a.y as f32 + a_half_height);
	let b_center = Vector2::new(b.x as f32 + b_half_width, b.y as f32 + b_half_height);

	let x_distance = a_center.x - b_center.x;
	let y_distance = a_center.y - b_center.y;
	let x_min_distance = a_half_width - b_half_width;
	let y_min_distance = a_half_height - b_half_height;

	if x_distance.abs() >= x_min_distance || y_distance >= y_min_distance {
		None
	} else {
		Some(Vector2::new(
			if x_distance > 0.0 { x_min_distance - x_distance } else { -x_min_distance - x_distance},
			if y_distance > 0.0 { y_min_distance - y_distance } else { -y_min_distance - y_distance},
		))
	}
}