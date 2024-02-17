//! Provid some basic object like [`Vec2`](stands for a 2 dimentional Vector).

#[cfg(feature = "vertexs")]
use crate::prelude::shape_elements::Vertex;
use serde::*;
use std::ops::Neg;
use crate::shape::shape_elements::Polygon;
use crate::shape::shape_elements::Animate;
use crate::shape::shape_elements::Shape;
use crate::shape::shape_elements::Style;
use core::f32::consts::PI;
use core::ops::Sub;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Add;

/// A simple 2 dimentional Vector usually stands for a point or an area starts from (0,0).
///
/// Vec2 have implied [`Add`] [`Mul`] [`Sub`] [`Div`] trait, which will add/mul/sub/div each component speartely.
/// Vec2 also implied [`Shape`] trait which stands for a single line form ```style.position to Vec2 + style.position```.
///
/// # Examples
/// 
/// ```
/// # use nablo_shape::prelude::Vec2;
///	let a = Vec2::new(1.,2.);
/// let b = Vec2::new(3.,4.);
/// let c = a + b;
/// assert_eq!(c, Vec2::new(4.,6.));
/// ```
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default, Copy)]
#[serde(default)]
pub struct Vec2 {
	/// follows svg cartesian.
	pub x: f32,
	/// follows svg cartesian.
	pub y: f32
}

impl Vec2 {
	/// same as Vec2::new(1.0, 1.0)
	pub const NOT_TO_SCALE: Vec2 = Vec2 {
		x: 1.0,
		y: 1.0
	};

	pub const ZERO: Vec2 = Vec2 {
		x: 0.0,
		y: 0.0
	};

	pub const INF: Vec2 = Vec2 {
		x: f32::INFINITY,
		y: f32::INFINITY
	};

	pub const NEG_INF: Vec2 = Vec2 {
		x: f32::NEG_INFINITY,
		y: f32::NEG_INFINITY
	};

	/// Get a new [`Vec2`].
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			x,
			y
		}
	}

	/// Get a new [`Vec2`] from same coordinates.
	pub fn same(input: f32) -> Self {
		Self {
			x: input,
			y: input
		}
	}

	/// Get a new [`Vec2`] with x value and y = 0.0.
	pub fn x(input: f32) -> Self {
		Self {
			x: input,
			y: 0.0
		}
	}

	/// Get a new [`Vec2`] with y value and x = 0.0.
	pub fn y(input: f32) -> Self {
		Self {
			y: input,
			x: 0.0
		}
	}

	/// get a new [`Vec2`] from polar coordinates
	pub fn polar(len: f32, angle: f32) -> Self {
		Self {
			x: len * angle.cos(),
			y: len * angle.sin(),
		}
	}

	/// Get a new [`Vec2`] from relative cartesian.
	pub fn from_size(x: f32, y: f32, size: Self) -> Self {
		Self::new(x,y) * size
	}

	/// The dot operation.
	pub fn dot(self, rhs: Self) -> f32 {
		self.x * rhs.x + self.y * rhs.y 
	}

	/// The cross operation, returns z coordinate
	pub fn cross(self, rhs: Self) -> f32 {
		self.x * rhs.y - self.y * rhs.x 
	}

	/// rotate a [`Vec2`] by angle rad.
	pub fn rotate(self, angle: f32) -> Self {
		Self {
			x: self.x * angle.cos() - self.y * angle.sin(),
			y: self.x * angle.sin() + self.y * angle.cos()
		}
	}

	/// scale a [`Vec2`] by given size. [`Vec2::NOT_TO_SCALE`] for not to scale
	pub fn scale(self, scale: Self) -> Self {
		self * scale
	}

	/// rotate a [`Vec2`] with a rotate center.
	pub fn rotate_with_center(self, angle: f32, rotate_center: Self) -> Self {
		(self - rotate_center).rotate(angle) + rotate_center
	}

	/// rotate a [`Vec2`] with a scale center.
	pub fn scale_with_center(self, scale: Self, scale_center: Self) -> Self {
		(self - scale_center).scale(scale) + scale_center
	}

	/// rotate and scale a [`Vec2`] with a scale center.
	pub fn transfrom_with_center(self,angle: f32, scale: Self, transform_origin: Self) -> Self {
		(self - transform_origin).rotate(angle).scale(scale) + transform_origin
	}

	/// get how long a [`Vec2`] is
	pub fn len(&self) -> f32 {
		self.p_norm(2.0)
	}

	/// caculate a [`Vec2`]'s [p norm](https://www.wolframalpha.com/input?i=p+norm)
	pub fn p_norm(&self, p: f32) -> f32 {
		(self.x.powf(p) + self.y.powf(p)).powf(1.0/p)
	}

	/// get svg vec expression.
	pub fn svg(&self) -> String {
		format!("{}, {}", self.x, self.y)
	}

	/// get the angle relate to horizental line. follows radius measure.
	pub fn angle(&self) -> f32 {
		if self.x == 0.0 {
			if self.y > 0.0 {
				return std::f32::consts::PI / 2.0
			}else {
				return std::f32::consts::PI / 2.0 * 3.0
			}
		}
		let angle = (self.y / self.x).abs().atan();
		if self.x > 0.0 {
			if self.y > 0.0 {
				return angle
			}else {
				return - angle + std::f32::consts::PI * 2.0
			}
		}else {
			if self.y > 0.0 {
				return - angle + std::f32::consts::PI
			}else {
				return angle + std::f32::consts::PI
			}
		}
	}

	/// for rendering stroke, return a [`Polygon`] with 4 points(actually a rotated rectangle) in given stroke width
	pub fn line(&self, stroke_width: f32) -> Polygon {
		let point0 = Vec2::same(0.0);
		let point1 = Vec2::polar(stroke_width, PI / 2.0 - self.angle());
		let point2 = *self;
		let point3 = point2 + point1;

		Polygon {
			points: vec!(point0, point1, point2, point3),
			..Default::default()
		}
	}

	/// check if current [`Vec2`] is on a line segment(defined by given two point)
	pub fn is_point_on_line(&self, point1: &Vec2, point2: &Vec2) -> bool {
		(*point1 - *self).cross(*point2 - *point1) == 0.0 && Area::new(*point1, *point2).is_point_inside(self)
	}

	/// check current point is inside an area. contains border
	pub fn is_inside(&self, area: Area) -> bool {
		area.is_point_inside(self)
	}
}

impl Shape for Vec2 {
	fn into_svg(&self, style: &Style) -> String {
		format!("<path d='M {} T {}' {} />" , style.position.svg(), (style.position + *self).svg(), style.svg_basic_settings())
	}

	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }

	fn get_area(&self, style: &Style) -> Area {
		[style.position.rotate_with_center(style.rotate, style.transform_origin).scale_with_center(style.size, style.transform_origin) , 
		(style.position + *self).rotate_with_center(style.rotate, style.transform_origin).scale_with_center(style.size, style.transform_origin)].into()
	}

	fn delta(&self, rhs: &Self) -> Self {
		*self - *rhs
	}

	fn change(&mut self, rhs: &Self) {
		*self = *self + *rhs
	}
}

impl Animate for Vec2 {
	fn animate(&mut self, id: usize, change_to: f32) {
		match id {
			0 => self.x = change_to,
			1 => self.y = change_to,
			_ => {},
		}
	}

	fn animate_len(&self) -> usize { 2 }
}

impl Add for Vec2 {
	type Output = Self;
	fn add(self, rhs: Vec2) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Mul for Vec2 {
	type Output = Self;
	fn mul(self, rhs: Vec2) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: f32) -> Self::Output {
		Vec2::new(self.x * rhs, self.y * rhs)
	}
}

impl Mul<Vec2> for f32 {
	type Output = Vec2;
	fn mul(self, rhs: Vec2) -> Self::Output {
		Vec2::new(self * rhs.x, self * rhs.y)
	}
}

impl Sub for Vec2 {
	type Output = Self;
	fn sub(self, rhs: Vec2) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl Div for Vec2 {
	type Output = Self;
	fn div(self, rhs: Vec2) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl Div<f32> for Vec2 {
	type Output = Self;
	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl Neg for Vec2 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl From<[f32; 2]> for Vec2 {
	fn from(input: [f32; 2]) -> Self {
		Self {
			x: input[0],
			y: input[1]
		}
	}
}

impl From<[usize; 2]> for Vec2 {
	fn from(input: [usize; 2]) -> Self {
		Self {
			x: input[0] as f32,
			y: input[1] as f32
		}
	}
}


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default, Copy)]
/// A simple 2 dimentional rectangle area usually stands for bounding box.
pub struct Area {
	#[serde(serialize_with = "serialize_area")]
	#[serde(deserialize_with = "deserialize_area")]
	pub area: [Vec2; 2]
}

fn serialize_area<S>(input: &[Vec2; 2], serializer: S) -> Result<S::Ok, S::Error> 
	where S: Serializer 
{
	let input = AreaInner {
		min: input[0],
		max: input[1]
	};
	input.serialize(serializer)
}

fn deserialize_area<'de, D>(deserializer: D) -> Result<[Vec2; 2], D::Error> 
	where D: Deserializer<'de>
{
	let de = AreaInner::deserialize(deserializer)?;
	Ok([de.min, de.max])
}

#[derive(serde::Deserialize, serde::Serialize)]
struct AreaInner {
	min: Vec2,
	max: Vec2
}

impl Into<Area> for [Vec2; 2] {
	fn into(self) -> Area {
		Area::new(self[0], self[1])
	}
}

impl Into<Area> for [[f32; 2]; 2] {
	fn into(self) -> Area {
		let point1: Vec2 = self[0].into();
		let point2: Vec2 = self[1].into();
		[point1,point2].into()
	}
}

impl Into<Area> for [f32; 4] {
	fn into(self) -> Area {
		let point1: Vec2 = [self[0], self[1]].into();
		let point2: Vec2 = [self[2], self[3]].into();
		[point1,point2].into()
	}
}

impl Area {
	pub const ZERO: Area = Area {
		area: [Vec2::ZERO; 2]
	};

	pub const INF: Area = Area {
		area: [Vec2::NEG_INF,  Vec2::INF]
	};

	/// check is this area contains nothing.
	pub fn is_empty(&self) -> bool {
		(self.area[1].x - self.area[0].x) <= 0.0 || (self.area[1].y - self.area[0].y) <= 0.0
	}

	/// check a point is inside this area. contains border
	pub fn is_point_inside(&self, point: &Vec2) -> bool {
		self.area[0].x <= point.x && 
		self.area[1].x >= point.x &&
		self.area[0].y <= point.y && 
		self.area[1].y >= point.y
	}

	/// get the width of this area
	pub fn width(&self) -> f32 {
		(self.area[1].x - self.area[0].x).abs()
	}

	/// get the height of this area
	pub fn height(&self) -> f32 {
		(self.area[1].y - self.area[0].y).abs()
	}

	/// get the left top point of this area
	pub fn left_top(&self) -> Vec2 {
		self.area[0]
	}

	/// get the right top point of this area
	pub fn right_top(&self) -> Vec2 {
		Vec2::new(self.area[1].x, self.area[0].y)
	}

	/// get the left bottom point of this area
	pub fn left_bottom(&self) -> Vec2 {
		Vec2::new(self.area[0].x, self.area[1].y)
	}

	/// get the left bottom point of this area
	pub fn right_bottom(&self) -> Vec2 {
		self.area[1]
	}

	/// get the center point of this area
	pub fn center(&self) -> Vec2 {
		(self.area[1] + self.area[0]) / 2.0
	}

	/// get the width and height of this area
	pub fn width_and_height(&self) -> Vec2 {
		self.area[1] - self.area[0]
	}

	/// get a new area, will sort two [`Vec2`] by coordinates
	pub fn new(point1: Vec2, point2: Vec2) -> Self {
		let mut max = Vec2::same(0.0);
		let mut min = Vec2::same(0.0);
		if point1.x > point2.x {
			max.x = point1.x;
			min.x = point2.x;
		}else {
			min.x = point1.x;
			max.x = point2.x;
		}
		if point1.y > point2.y {
			max.y = point1.y;
			min.y = point2.y;
		}else {
			min.y = point1.y;
			max.y = point2.y;
		}
		Self {
			area: [min, max]
		}
	}

	/// get a new area from sigle point
	pub fn new_with_origin(point: Vec2) -> Self {
		Self {
			area: [Vec2::new(0.0,0.0), point]
		}
	}

	/// check if two area have cross
	pub fn is_cross(&self, other: &Area) -> bool {
		self.is_point_inside(&other.left_top()) | 
		self.is_point_inside(&other.right_top()) |
		self.is_point_inside(&other.left_bottom()) |
		self.is_point_inside(&other.right_bottom()) 
	}

	/// find cross part of a area, returns [`Area::ZERO`] if not have cross part
	pub fn cross_part(&self, other: &Area) -> Area {
		if self.is_inside(other) {
			return *other
		}
		if other.is_inside(self) {
			return *self
		}
		fn min(input1: f32, input2: f32) -> f32 {
			if input1 < input2 {
				return input1;
			}
			input2
		}

		fn max(input1: f32, input2: f32) -> f32 {
			if input1 < input2 {
				return input2;
			}
			input1
		}

		let area = Area::new(
			Vec2::new(max(self.area[0].x, other.area[0].x), max(self.area[0].y, other.area[0].y)),
			Vec2::new(min(self.area[1].x, other.area[1].x), min(self.area[1].y, other.area[1].y)),
		);

		if area.is_empty() {
			Area::ZERO
		}else {
			area
		}
	}

	/// combine two areas
	pub fn combine(&mut self, other: &Area) {
		fn min(input1: f32, input2: f32) -> f32 {
			if input1 < input2 {
				return input1;
			}
			input2
		}

		fn max(input1: f32, input2: f32) -> f32 {
			if input1 < input2 {
				return input2;
			}
			input1
		}
		
		if self.is_empty() {
			*self = *other
		}else if other.is_empty() {}
		else {
			*self = Area::new(
				Vec2::new(min(self.area[0].x, other.area[0].x), min(self.area[0].y, other.area[0].y)),
				Vec2::new(max(self.area[1].x, other.area[1].x), max(self.area[1].y, other.area[1].y)),
			)
		}	
	}

	/// check the other [`Area`] is inside current [`Area`]
	pub fn is_inside(&self, other: &Area) -> bool {
		self.is_point_inside(&other.left_top()) & 
		self.is_point_inside(&other.right_top()) &
		self.is_point_inside(&other.left_bottom()) &
		self.is_point_inside(&other.right_bottom()) 
	}

	/// get cross point with border. [`Option::None`] for no cross
	pub fn find_cross(&self, point1: &Vec2, point2: &Vec2) -> Option<Vec2> {
		if self.is_point_inside(point1) != self.is_point_inside(point2) {
			fn is_point_on_line(cross: &Option<Vec2>, point1: &Vec2, point2: &Vec2) -> bool {
				if let Some(t) = cross {
					return t.is_point_on_line(point1, point2)
				}
				false
			}

			let cross_points = [
				line_cross(point1, point2, &self.left_top(), &self.right_top()),
				line_cross(point1, point2, &self.left_top(), &self.left_bottom()),
				line_cross(point1, point2, &self.right_bottom(), &self.right_top()),
				line_cross(point1, point2, &self.right_bottom(), &self.left_bottom()),
			];

			for point in cross_points {
				if is_point_on_line(&point, point1, point2) {
					return point
				}
			}
		}

		None
	}

	/// transform a [`Area`] with given [`Style`]
	pub fn transform(self, style: &Style) -> Self {
		let points = [self.left_top(), self.left_bottom(), self.right_top(), self.right_bottom()];
		let points: Vec<Vec2> = points.into_iter().map(|point| {
			point.transfrom_with_center(style.rotate, style.size, style.transform_origin)
		}).collect();
		let mut min = Vec2::same(f32::INFINITY);
		let mut max = Vec2::same(f32::NEG_INFINITY);
		for point in &points {
			if point.x < min.x {
				min.x = point.x
			}
			if point.y < min.y {
				min.y = point.y
			}
			if point.x > max.x {
				max.x = point.x
			}
			if point.y > max.y {
				max.y = point.y
			}
		}
		Area::new(min + style.position, max + style.position)
	}

	/// move a area to a new place
	#[inline]
	pub fn move_delta_to(&mut self, position: Vec2) {
		self.area[0] = self.area[0] + position;
		self.area[1] = self.area[1] + position;
	}

	/// shrink a area from given size
	#[inline]
	pub fn shrink(&self, shrink: Vec2) -> Area {
		[self.area[0] + shrink,
		self.area[1] - shrink].into()
	}
}

/// find the cross point for two lines, [`Option::None`] for no cross
pub fn line_cross(l1_point1: &Vec2, l1_point2: &Vec2, l2_point1: &Vec2, l2_point2: &Vec2) -> Option<Vec2> {
	let a1 = l1_point2.y - l1_point1.y;
	let b1 = l1_point1.x - l1_point2.x;
	let c1 = l1_point1.x * l1_point2.y  - l1_point1.y * l1_point2.x;

	let a2 = l2_point2.y - l2_point1.y;
	let b2 = l2_point1.x - l2_point2.x;
	let c2 = l2_point1.x * l2_point2.y  - l2_point1.y * l2_point2.x;

	let det = a1 * b2 - a2 * b1;
	if det == 0.0 {
		return None
	}else {
		let d1 = c1 * b2 - c2 * b1;
		let d2 = a1 * c2 - a2 * c1;
		return Some(Vec2::new(d1 / det, d2 / det))
	}
}