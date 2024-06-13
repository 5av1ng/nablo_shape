//! Provid some basic abstraction of animation.

use time::Duration;
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
	pub start_time: Duration,
	pub start_value: f32,
	pub linkers: Vec<Linker>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
/// a enum refers to animation link types
pub enum AnimationLinker {
	/// the control point is standardlized
	Bezier(Vec2, Vec2),
	Power(f32),
	#[default] Linear,
	Mutation,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
/// a struct refers to animation points
pub struct Linker {
	pub end_value: f32,
	pub sustain_time: Duration,
	pub linker: AnimationLinker,
}

impl Default for StyleToAnimate {
	fn default() -> Self {
		Self::Style(0)
	}
}

impl Linker {
	/// caculate a value inside a linker, x is standardlized
	fn caculate(&self, x: f32, start_value: f32) -> f32 {
		match self.linker {
			AnimationLinker::Bezier(control_point_one, control_point_two) => {
				let t: f32;
				let mut left = 0.0;
				let mut right = 1.0;
				loop {
					let middle = (left + right)/2.0;
					let result = 3.0 * middle * (1.0 - middle) * (1.0 - middle) * control_point_one.x +
					3.0 * middle * middle * (1.0 - middle) * control_point_two.x +
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
				let y = 3.0 * t * (1.0 - t) * (1.0 - t) * control_point_one.y +
						3.0 * t * t * (1.0 - t) * control_point_two.y +
						t * t * t;

				return start_value + (self.end_value - start_value) * y;
			},
			AnimationLinker::Power(factor) => {
				return x.powf(factor) * (self.end_value - start_value) + start_value;
			},
			AnimationLinker::Linear => {
				return x * (self.end_value - start_value) + start_value;
			},
			AnimationLinker::Mutation => {
				return start_value;
			}
		}
	}
}

impl Animation {
	/// create a [`Animation`] with `start_value = 0.0` and `end_value = 1.0`
	pub fn new_standard(sustain_time: Duration, control_point_one: Vec2, control_point_two: Vec2) -> Self {
		Self {
			start_value: 0.0,
			start_time: Duration::ZERO,
			linkers: vec!(Linker {
				end_value: 1.0,
				sustain_time,
				linker: AnimationLinker::Bezier(control_point_one, control_point_two)
			})
		}
	}

	/// get how long the animation sustains
	#[inline]
	pub fn len(&self) -> Duration {
		let mut total = Duration::ZERO;
		for a in &self.linkers {
			total = total + a.sustain_time;
		}
		total
	}

	/// get end time of the animatio
	#[inline]
	pub fn end_time(&self) -> Duration {
		self.len() + self.start_time
	}

	/// check if current animation have same time with other 
	#[inline]
	pub fn is_cross(&self, other: &Animation) -> bool {
		(self.end_time() >= other.start_time && self.start_time <= other.start_time) || (other.end_time() >= self.start_time && other.start_time <= self.start_time)
	}

	/// get absolute time of every linkers
	#[inline]
	pub fn stages(&self) -> Vec<Duration> {
		let mut back = vec!(self.start_time);
		let mut len = self.start_time;
		for linker in &self.linkers {
			len = len + linker.sustain_time;
			back.push(len);
		}
		back
	}

	/// get how much linkers do this animation have
	#[inline]
	pub fn linkers_len(&self) -> usize {
		self.linkers.len()
	} 

	/// check if this animation has no linker
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.linkers_len() == 0
	}

	/// caculate animation [`Option::None`] for haven't start or already end.
	pub fn caculate(&self, current_time: &Duration) -> Option<f32> {
		// we'll standardlized time value
		let stages = self.stages();
		let mut end = None;
		for i in 0..(stages.len() - 1) {
			if current_time < &stages[i + 1] && current_time >= &stages[i] {
				let x = (*current_time - stages[i]) / (stages[i + 1] - stages[i]);
				let start_value = if i == 0 {
					self.start_value
				}else {
					self.linkers[i - 1].end_value
				};
				let caculate = self.linkers[i].caculate(x as f32, start_value);
				end = Some(caculate);
			}
		}
		
		end
	}

	/// get end value of current animation
	pub fn end_value(&self) -> f32 {
		if self.linkers.len() == 0 {
			self.start_value
		}else {
			self.linkers[self.linkers.len() - 1].end_value
		}
	}

	/// get minimal value of current animation
	pub fn min_value(&self) -> f32 {
		let mut min = self.start_value;
		for a in &self.linkers {
			if a.end_value < min {
				min = a.end_value
			}
		}
		min
	}

	/// get maxium value of current animation
	pub fn max_value(&self) -> f32 {
		let mut max = self.start_value;
		for a in &self.linkers {
			if a.end_value > max {
				max = a.end_value
			}
		}
		max
	}

	/// use absolute time to add a new point, will replace it when the differrnce between time and absolute time is lower than Duration::milliseconds(1)
	pub fn insert_point(&mut self, time: Duration, end_value: f32, linker: AnimationLinker) {
		self.insert_point_with_epsilon(time, end_value, linker, Duration::milliseconds(1));
	}

	/// use absolute time to add a new point, will replace it when the differrnce between time and absolute time is lower than epsilon
	pub fn insert_point_with_epsilon(&mut self, time: Duration, end_value: f32, linker: AnimationLinker, epsilon: Duration) {
		if (time - self.start_time).abs() <= epsilon {
			self.start_value = end_value;
			if !self.linkers.is_empty() {
				self.linkers[0].sustain_time = self.start_time + self.linkers[0].sustain_time - time;
			}
			self.start_time = time;
			return;
		}
		if time < self.start_time {
			let new_linker = Linker {
				end_value: self.start_value,
				sustain_time: self.start_time - time,
				linker,
			};
			self.start_time = time;
			self.start_value = end_value;
			self.linkers.insert(0, new_linker);
			return;
		}

		let len = self.len() + self.start_time;
		if time - len > epsilon {
			let new_linker = Linker {
				end_value,
				sustain_time: time - len,
				linker,
			};
			self.linkers.push(new_linker);
			return;
		}

		let mut last_time = self.start_time;
		for i in 0..self.linkers.len() {
			if (time - last_time - self.linkers[i].sustain_time).abs() <= epsilon {
				self.linkers[i].linker = linker;
				if i + 1 < self.linkers.len() {
					self.linkers[i + 1].sustain_time = last_time + self.linkers[i + 1].sustain_time + self.linkers[i].sustain_time - time;
				}
				self.linkers[i].sustain_time = (time - last_time).abs();
				self.linkers[i].end_value = end_value;
				break;
			}
			if time - last_time < self.linkers[i].sustain_time {
				let new_linker = Linker {
					end_value,
					sustain_time: time - last_time,
					linker,
				};
				self.linkers[i].sustain_time = self.linkers[i].sustain_time - time + last_time;
				self.linkers.insert(i, new_linker);
				break;
			}
			last_time = last_time + self.linkers[i].sustain_time;
		}
	}

	/// use absolute time to delete a point, will delete nearest and difference < Duration::milliseconds(150) point, will reset animation to default if there is no point to be deleted
	pub fn remove_point(&mut self, time: Duration) {
		self.remove_point_with_epsilon(time, Duration::milliseconds(150))
	}

	/// use absolute time to delete a point, will delete nearest and difference < epsilon point, will reset animation to default if there is no point to be deleted
	pub fn remove_point_with_epsilon(&mut self, time: Duration, epsilon: Duration) {
		let stages = self.stages();
		let mut nearest_id = None;
		for (i, stage) in stages.iter().enumerate() {
			let delta_time = (time - *stage).abs();
			if delta_time <= epsilon {
				if let Some((_, delta)) = nearest_id {
					if delta > delta_time {
						nearest_id = Some((i, delta_time));
					}
				}else {
					nearest_id = Some((i, delta_time));
				}
			}
		}
		if let Some((id, _)) = nearest_id {
			if id == 0 {
				if self.linkers.is_empty() {
					*self = Default::default();
				}else {
					self.start_value = self.linkers[0].end_value;
					self.start_time = self.linkers[0].sustain_time + self.start_time;
					self.linkers.remove(0);
				}
			}else if id < self.linkers.len() {
				self.linkers[id].sustain_time =  self.linkers[id - 1].sustain_time + self.linkers[id].sustain_time;
				self.linkers.remove(id - 1);
			}else {
				self.linkers.pop();
			}
		}
	}

	/// combine two animations, will replace current animation with other animation's corresponding areas and leave other anmation empty, using linker if two animations have gaps.
	pub fn combine(&mut self, other: &mut Animation, linker: AnimationLinker) {
		if self.is_cross(other) {
			if self.start_time > other.start_time { 
				let mut linker_id = 0;
				let mut last_stage = other.start_time;
				for (i, stage) in other.stages().iter().enumerate() {
					if *stage > self.start_time {
						other.linkers[i] = Linker {
							end_value: self.start_value,
							sustain_time: self.start_time - last_stage,
							linker,
						};
						linker_id = i;
						break;
					}
					last_stage = *stage;
				}
				for linker in &self.linkers {
					if let Some(t) = other.linkers.get_mut(linker_id) {
						*t = linker.clone();
					}else {
						other.linkers.push(linker.clone());
					}
					linker_id += 1;
				}
				*self = other.clone();
				other.linkers.clear();
			}else {
				let mut linker_id = 0;
				let mut last_stage = self.start_time;
				for (i, stage) in self.stages().iter().enumerate() {
					if *stage > other.start_time {
						self.linkers[i] = Linker {
							end_value: other.start_value,
							sustain_time: other.start_time - last_stage,
							linker,
						};
						linker_id = i;
						break;
					}
					last_stage = *stage;
				}
				for linker in &other.linkers {
					if let Some(t) = self.linkers.get_mut(linker_id) {
						*t = linker.clone();
					}else {
						self.linkers.push(linker.clone())
					}
					linker_id += 1;
				}
				other.linkers.clear();
			}
		}else {
			if self.start_time > other.start_time {
				other.insert_point(self.start_time - other.end_time(), self.start_value, linker);
				other.linkers.append(&mut self.linkers);
				*self = other.clone();
				other.linkers.clear();
			}else {
				self.insert_point(other.start_time - self.end_time(),other.start_value ,linker);
				self.linkers.append(&mut other.linkers);
			}
		}
	}
}