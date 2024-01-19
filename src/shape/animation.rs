//! Provid some basic abstraction of animation.

use time::Duration;
use std::ops::Add;
use std::ops::Sub;
use crate::math::Vec2;

// TODO: make this value changeable.
const ACCURACY: f32 = 0.01;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
/// stand for what to animate
pub enum StyleToAnimate {
	/// standard style id. See more at [`crate::shape::Style`].
	Style(usize),
	/// shape animation id. See more at [`crate::shape::Shape`].
	Id(usize)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a simple animation tool
pub struct Animation {
	// what we save is nano sec.
	pub start_time: Duration,
	// what we save is nano sec.
	pub sustain_time: Duration,
	/// the control point is standardlized
	pub control_point_one: Vec2,
	/// the control point is standardlized
	pub control_point_two: Vec2,
	pub start_value: f32,
	pub end_value: f32,
}

impl Default for StyleToAnimate {
	fn default() -> Self {
		Self::Style(0)
	}
}

impl Add for Animation {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			start_time: self.start_time + self.start_time,
			sustain_time: self.sustain_time + self.sustain_time,
			control_point_one: self.control_point_one + other.control_point_one,
			control_point_two: self.control_point_two + other.control_point_two,
			start_value: self.start_value + self.start_value,
			end_value: self.end_value + self.end_value,
		}
	}
}

impl Sub for Animation {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			start_time: self.start_time - self.start_time,
			sustain_time: self.sustain_time - self.sustain_time,
			control_point_one: self.control_point_one - other.control_point_one,
			control_point_two: self.control_point_two - other.control_point_two,
			start_value: self.start_value - self.start_value,
			end_value: self.end_value - self.end_value,
		}
	}
}

impl Animation {
	/// to check two if [`Animation`] have time cross.
	pub fn is_cross(&self, other: &Self) -> bool {
		!(self.start_time + self.sustain_time < other.start_time || self.start_time > other.start_time + other.sustain_time )
	}

	/// create a [`Animation`] with `start_value = 0.0` and `end_value = 1.0`
	pub fn new_standard(sustain_time: Duration, control_point_one: Vec2, control_point_two: Vec2) -> Self {
		Self {
			start_value: 0.0,
			end_value: 1.0,
			start_time: Duration::ZERO,
			sustain_time: sustain_time,
			control_point_one,
			control_point_two,
		}
	}

	/// caculate animation [`Option::None`] for haven't start.
	pub fn caculate(&self, current_time: &Duration) -> Option<f32>{
		// we'll standardlized time value
		let x = (*current_time - self.start_time).as_seconds_f32() / self.sustain_time.as_seconds_f32();

		if x > 1.0 || x < 0.0 {
			return None
		}

		Some(self.caculate_standardlized(x))
	}

	/// caculate with given standaedlized time
	pub fn caculate_standardlized(&self, x: f32) -> f32{
		let t: f32;
		let mut left = 0.0;
		let mut right = 1.0;
		loop {
			let middle = (left + right)/2.0;
			let result = 3.0 * middle * (1.0 - middle) * (1.0 - middle) * self.control_point_one.x +
			3.0 * middle * middle * (1.0 - middle) * self.control_point_two.x +
			middle * middle * middle;
			if result == x {
				t = middle;
				break;
			}else if result < x {
				left = middle
			}else {
				right = middle
			}
			if (right - left) < ACCURACY {
				t = middle;
				break;
			}
		}

		let y = 3.0 * t * (1.0 - t) * (1.0 - t) * self.control_point_one.y +
				3.0 * t * t * (1.0 - t) * self.control_point_two.y +
				t * t * t;

		self.start_value + (self.end_value - self.start_value) * y
	}
}