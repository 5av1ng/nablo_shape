//! Provid some basic elements for shape.
//!
//! You can use your shape by adding the [`Shape`] trait.
//! Or just using [`Svg`] to print your svg with out animation.

// use once_cell::sync::Lazy;
// use fontdue::Font;
use std::ops::Mul;
use crate::prelude::ShapeMask;
use std::ops::IndexMut;
use std::ops::Index;
use crate::shape::Area;
use std::f32::consts::PI;
use std::ops::Sub;
use std::ops::Add;
use std::fmt::Debug;
use crate::math::Vec2;
use rayon::prelude::*;

/// refer to css.
pub const EM: f32 = 16.0;
/// TODO: font change support
// static FONT: Lazy<Font> = Lazy::new(|| {fontdue::Font::from_bytes(include_bytes!("../../font.ttf") as &[u8], Default::default()).expect("loading font failed")});

/// a trait for a shape
pub trait Shape: Default + Clone + Debug + PartialEq + Animate {
	/// for some reason, we will process shape as svg.
	fn into_svg(&self, style: &Style) -> String;
	/// translate a shape to vertexs
	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, style: &Style, size: Vec2) -> (Vec<Vertex>, Vec<u32>);
	/// For UI framework to ensure where the shape is(using top left point and bottom right point to stand for a rectangle).
	fn get_area(&self, style: &Style) -> Area;
	/// tell what have changed which you want be changed in mulitiselection to shapoist.
	fn delta(&self, compare: &Self) -> Self;
	/// apply what have changed to other in mulitiselection
	fn change(&mut self, delta: &Self);
}

/// if we want to animate some thing
pub trait Animate {
	/// what you want animate using [`usize`] to represent your attribute, if the id was larger than your number of attribute, then it should done nothing.
	fn animate(&mut self, _: usize , _: f32) {}
	/// how much attributes that can be animated does your shape have?
	fn animate_len(&self) -> usize { 0 }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, serde::Deserialize, serde::Serialize, PartialEq, Default)]
/// where should we draw using pipline?
pub struct Vertex {
	pub position: [f32; 3],
	pub color: [f32; 4],
}

impl From<(Vec2, [u8;4])> for Vertex {
	fn from(input: (Vec2, [u8;4])) -> Self {
		let (position, color) = input;
		Self {
			position: [position.x, position.y, 0.0],
			color: [color[0] as f32 / 255.0,color[1] as f32 / 255.0,color[2] as f32 / 255.0,color[3] as f32 / 255.0],
		}
	}
}

impl From<(Vec2, Color)> for Vertex {
	fn from(input: (Vec2, Color)) -> Self {
		let (position, color) = input;
		Self {
			position: [position.x, position.y, 0.0],
			color: color.normalized(),
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
/// for general styles of a shape
pub struct Style {
	/// Where we are, id = 0 or 1 represents position.x or position.y.
	pub position: Vec2,
	/// rotate and scale center, id = 2 or 3 represents transform_origin.x or transform_origin.y.
	pub transform_origin: Vec2,
	/// follows radian measure, id = 4 repensents this attribute.
	pub rotate: f32,
	/// normally is [`crate::math::Vec2::NOT_TO_SCALE`]. Will **not** affect stroke width. id = 5 or 6 represents size.x or size.y.
	pub size: Vec2,
	/// stands for rgba, id = 7, 8, 9 or 10 represents ```fill[0]``` ```fill[1]``` ```fill[2]``` or ```fill[3]```.
	pub fill: Color,
	/// id = 11 repensents this attribute.
	pub stroke_width: f32,
	/// stands for rgba, id = 12, 13, 14 or 15 represents ```fill[0]``` ```fill[1]``` ```fill[2]``` or ```fill[3]```.
	pub stroke_color: Color,
	/// where should we draw?
	pub layer: Layer,
	/// you only need to show shapes inside this clip
	pub clip: Area
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq, Default, Copy, PartialOrd, Ord, Hash)]
/// where should we draw?
pub enum Layer {
	#[default] Background = 0,
	Bottom = 1,
	Middle = 2,
	Foreground = 3,
	ToolTips = 4,
	Debug = 5
}

impl Layer {
	/// get layer id
	pub fn into_id(&self) -> usize {
		match self {
			Self::Background => 0,
			Self::Bottom => 1,
			Self::Middle => 2,
			Self::Foreground => 3,
			Self::ToolTips => 4,
			Self::Debug => 5
		}
	}

	/// returns lower Layer compare to self
	pub fn lower(&self) -> Self {
		match &self {
			Self::Background => Self::Background,
			Self::Bottom => Self::Background,
			Self::Middle => Self::Bottom,
			Self::Foreground => Self::Middle,
			Self::ToolTips => Self::Foreground,
			Self::Debug => Self::ToolTips,
		}
	}
}

/// stands for r g b a unpremulitiplied.
#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize, Copy)]
pub struct Color {
	pub color: [u8; 4]
}

impl From<[u8; 4]> for Color {
	fn from(color: [u8; 4]) -> Self {
		Self {
			color
		} 
	}
}

impl From<[u8; 3]> for Color {
	fn from(color: [u8; 3]) -> Self {
		Self {
			color: [color[0], color[1], color[2], 255]
		} 
	}
}

impl From<u8> for Color {
	fn from(color: u8) -> Self {
		Self {
			color: [color, color, color, 255]
		} 
	}
}

impl From<f32> for Color {
	fn from(color: f32) -> Self {
		Self {
			color: [(compress(color) * 255.0) as u8,
			(compress(color) * 255.0) as u8,
			(compress(color) * 255.0) as u8,
			255]
		} 
	}
}

impl From<[f32; 4]> for Color {
	fn from(color: [f32; 4]) -> Self {
		let color = [(compress(color[0]) * 255.0) as u8,
		(compress(color[1]) * 255.0) as u8,
		(compress(color[2]) * 255.0) as u8,
		(compress(color[3]) * 255.0) as u8];
		Self {
			color
		} 
	}
}

impl From<[f32; 3]> for Color {
	fn from(color: [f32; 3]) -> Self {
		let color = [(compress(color[0]) * 255.0) as u8,
		(compress(color[1]) * 255.0) as u8,
		(compress(color[2]) * 255.0) as u8,
		255];
		Self {
			color
		} 
	}
}

impl Index<usize> for Color {
	type Output = u8;
	fn index(&self, index: usize) -> &Self::Output {
		&self.color[index]
	}
}

impl IndexMut<usize> for Color {
	fn index_mut(&mut self, index: usize) -> &mut u8 {
		&mut self.color[index]
	}
}

impl Add<Color> for Color {
	type Output = Color;
	fn add(self, rhs: Color) -> Self::Output {
		[(self[0] as f32 + rhs[0] as f32) / 255.0,
		(self[1] as f32 + rhs[1] as f32) / 255.0,
		(self[2] as f32 + rhs[2] as f32) / 255.0,
		(self[3] as f32 + rhs[3] as f32) / 255.0].into()
	}
}

impl Sub<Color> for Color {
	type Output = Color;
	fn sub(self, rhs: Color) -> Self::Output {
		[(self[0] as f32 - rhs[0] as f32) / 255.0,
		(self[1] as f32 - rhs[1] as f32) / 255.0,
		(self[2] as f32 - rhs[2] as f32) / 255.0,
		(self[3] as f32 - rhs[3] as f32) / 255.0].into()
	}
}

impl Mul<f32> for Color {
	type Output = Color;
	fn mul(self, input: f32) -> Self::Output {
		[(self[0] as f32 * input) / 255.0,
		(self[1] as f32 * input) / 255.0,
		(self[2] as f32 * input) / 255.0,
		(self[3] as f32 * input) / 255.0].into()
	}
}

impl Mul<Color> for f32 {
	type Output = Color;
	fn mul(self, input: Color) -> Self::Output {
		[(input[0] as f32 * self) / 255.0,
		(input[1] as f32 * self) / 255.0,
		(input[2] as f32 * self) / 255.0,
		(input[3] as f32 * self) / 255.0].into()
	}
}

impl Color {
	pub const WHITE: Color = Self{
		color: [255,255,255,255]
	};

	pub const BLACK: Color = Self{
		color: [0,0,0,255]
	};

	/// get normalied coloe
	pub fn normalized(&self) -> [f32; 4] {
		[(self[0] as f32 / 255.0).powf(2.2),
		(self[1] as f32 / 255.0).powf(2.2),
		(self[2] as f32 / 255.0).powf(2.2),
		(self[3] as f32 / 255.0).powf(2.2)]
	}

	/// change current color, let them become brighter or darker
	pub fn brighter(self, factor: f32) -> Self {
		let alpha = self[3];
		let mut back;
		if factor > 0.0 {
			back = self + factor.abs().into();
		}else {
			back = self - factor.abs().into();
		}
		back[3] = alpha;
		back
	}

	/// change current color's alpha.
	pub fn set_alpha(self, alpha: u8) -> Self {
		let mut back = self;
		back[3] = alpha;
		back
	}
}

impl Default for Style {
	fn default() -> Self {
		Self {
			position: Vec2::default(),
			transform_origin: Vec2::default(),
			rotate: 0.0,
			size: Vec2::NOT_TO_SCALE,
			fill: [255,255,255,255].into(),
			stroke_width: 0.0,
			stroke_color: [0; 4].into(),
			layer: Layer::Bottom,
			clip: Area::ZERO
		}
	}
}

impl Style {
	/// such as color stroke etc.
	pub fn svg_basic_settings(&self) -> String {
		format!("stroke-width=\"{}\" stroke=\"rgb({},{},{})\" stroke-opacity=\"{}\" fill=\"rgb({},{},{})\"  fill-opacity=\"{}\" transform-origin=\"{},{}\" transform=\"rotate({})\" transform=\"scale({}, {})\"",
			self.stroke_width,
			self.stroke_color[0],
			self.stroke_color[1],
			self.stroke_color[2],
			self.stroke_color[3] as f32 / 255.0,
			self.fill[0],
			self.fill[1],
			self.fill[2],
			self.fill[3] as f32 / 255.0,
			self.transform_origin.x,
			self.transform_origin.y,
			self.rotate / std::f32::consts::PI * 360.0,
			self.size.x,
			self.size.y,
			)
	}

	/// same as [`Animate`]
	pub fn animate(&mut self, id: usize, change_to: f32) {
		fn compress(input: f32) -> f32 {
			if input > 1.0 {
				1.0
			}else if input < 0.0 {
				0.0
			}else {
				input
			}
		}

		match id {
			0 => self.position.x = change_to,
			1 => self.position.y = change_to,
			2 => self.transform_origin.x = change_to,
			3 => self.transform_origin.y = change_to,
			4 => self.rotate = change_to,
			5 => self.size.x = change_to,
			6 => self.size.y = change_to,
			7 => self.fill[0] = (compress(change_to) * 255.0) as u8,
			8 => self.fill[1] = (compress(change_to) * 255.0) as u8,
			9 => self.fill[2] = (compress(change_to) * 255.0) as u8,
			10 => self.fill[3] = (compress(change_to) * 255.0) as u8,
			11 => self.stroke_width = change_to,
			12 => self.stroke_color[0] = (compress(change_to) * 255.0) as u8,
			13 => self.stroke_color[1] = (compress(change_to) * 255.0) as u8,
			14 => self.stroke_color[2] = (compress(change_to) * 255.0) as u8,
			15 => self.stroke_color[3] = (compress(change_to) * 255.0) as u8,
			_ => {}
		}
	}

	pub fn len(&self) -> usize { 16 }
}

impl Add for Style {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			position: self.position + rhs.position,
			transform_origin: self.transform_origin + rhs.position,
			rotate: self.rotate + rhs.rotate,
			size: self.size + rhs.size,
			fill: rhs.fill,
			stroke_width: self.stroke_width + rhs.stroke_width,
			stroke_color: rhs.stroke_color,
			..self
		}
	}
}

impl Sub for Style {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			position: self.position - rhs.position,
			transform_origin: self.transform_origin - rhs.position,
			rotate: self.rotate - rhs.rotate,
			size: self.size - rhs.size,
			stroke_width: self.stroke_width - rhs.stroke_width,
			..self
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a circle, uh actually a ellipse if you have applied size.
pub struct Circle {
	pub radius: f32,
}

impl Shape for Circle {
	fn into_svg(&self, style: &Style) -> String {
		format!("<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {}/>", 
			style.position.x, 
			style.position.y,
			self.radius * style.size.x,
			self.radius * style.size.y,
			style.svg_basic_settings(),
			)
	}

	fn get_area(&self, style: &Style) -> Area {
		Rect {
			width_and_height: Vec2::same(2.0 * self.radius),
			rounding: Vec2::same(self.radius)
		}.get_area(style)
	}

	fn delta(&self, rhs: &Self) -> Self { 
		Self {
			radius: self.radius - rhs.radius
		}
	}

	fn change(&mut self, rhs: &Self) {
		*self = Self {
			radius: self.radius + rhs.radius
		}
	}

	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Animate for Circle {
	fn animate(&mut self, id: usize, change_to: f32) {
		match id {
			0 => self.radius = change_to,
			_ => {},
		}
	}

	fn animate_len(&self) -> usize { 1 }
}

impl Circle {
	/// sample a point on circle
	pub fn sample(&self, t: f32) -> Vec2 {
		Vec2::new((2.0*PI*t).cos(), (2.0*PI*t).sin()) * self.radius
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a rectangle.
pub struct Rect {
	pub width_and_height: Vec2,
	/// stands for rx and ry.
	pub rounding: Vec2,
}

impl Shape for Rect {
	fn into_svg(&self, style: &Style) -> String {
		format!("<rect x=\"{}\" y=\"{}\" rx=\"{}\" ry=\"{}\" {} />", 
			style.position.x, 
			style.position.y,
			self.rounding.x,
			self.rounding.y,
			style.svg_basic_settings()
			)
	}

	fn get_area(&self, style: &Style) -> Area {
		Area::new(Vec2::ZERO, self.width_and_height).transform(style)
	}

	fn delta(&self, rhs: &Self) -> Self {
		Self {
			width_and_height: self.width_and_height - rhs.width_and_height,
			rounding: self.rounding - rhs.rounding
		}
	}

	fn change(&mut self, rhs: &Self) { 
		*self = Self {
			width_and_height: self.width_and_height + rhs.width_and_height,
			rounding: self.rounding + rhs.rounding
		} 
	}

	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Animate for Rect {
	fn animate(&mut self, id: usize, change_to: f32) {
		match id {
			0 => self.width_and_height.x = change_to,
			1 => self.width_and_height.y = change_to,
			2 => self.rounding.x = change_to,
			3 => self.rounding.y = change_to,
			_ => {}
		}
	}

	fn animate_len(&self) -> usize { 4 }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a text. cant be rotated. cant be scale with transform origin. cant even caculate area correctly.
pub struct Text {
	pub text: String,
	/// [`Option::None`] stands for the whole screen
	pub text_width: Option<f32>,
	/// [`Option::None`] stands for the whole screen
	pub text_height: Option<f32>,
	/// such as bold or italic
	pub text_style: TextStyle
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
/// decides how to render text
pub struct TextStyle {
	pub is_bold: bool,
	pub is_italic: bool
}

impl TextStyle {
	/// set bold for current text stlye
	pub fn set_bold(self, is_bold: bool) -> Self {
		Self {
			is_bold,
			..self
		}
	}

	/// set italic for current text stlye
	pub fn set_italic(self, is_italic: bool) -> Self {
		Self {
			is_italic,
			..self
		}
	}
}

impl Shape for Text {
	fn into_svg(&self, style: &Style) -> String {
		format!("<text x=\"{}\" y=\"{}\" {}>{}</text>", 
			style.position.x, 
			style.position.y,
			style.svg_basic_settings(),
			self.text
			)
	}

	fn get_area(&self, style: &Style) -> Area {
		// since we haven't provid font change functions, this would not associated with font.
		let em = EM * style.size.len() / 2.0_f32.sqrt();
		let mut x = vec!(0.0);
		let mut line = 1.0;
		for i in 0..utf8_slice::len(&self.text) {
			let width;
			let text = utf8_slice::slice(&self.text, i, i + 1).chars().next().unwrap();
			if (text >= '一' && text <= '龥') || text == '●'  {
				width = em * 0.8 * 1.5;
			}else {
				width = em * 0.8;
			};
			if text == '\n' {
				line = line + 1.0;
				x.push(0.0);
			}else {
				x[(line - 1.0) as usize] = x[(line - 1.0) as usize] + width;
			}
		};
		Area::new(Vec2::ZERO, Vec2::new(*x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(), em * line)).transform(style)
	}

	fn delta(&self, _: &Self) -> Self { self.clone() }
	fn change(&mut self, _: &Self) {}
	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Animate for Text {
	fn animate(&mut self, _: usize, _: f32) {}
	fn animate_len(&self) -> usize { 0 }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a cubic bezier curve.
pub struct CubicBezier {
	pub points: [Vec2; 4],
	pub if_close: bool,
}

impl CubicBezier {
	/// using parametric equation to get a point on curve withour transform, t should in [0..=1].
	pub fn sample(&self, t: f32) -> Vec2 {
		let h = 1.0 - t;
		let a = t * t * t;
		let b = 3.0 * t * t * h;
		let c = 3.0 * t * h * h;
		let d = h * h * h;
		self.points[3] * a
			+ self.points[2] * b
			+ self.points[1] * c
			+ self.points[0] * d
	}
}

impl Shape for CubicBezier {
	fn into_svg(&self, style: &Style) -> String {
		let z;
		if self.if_close {
			z = String::from("Z")
		}else {
			z = String::new()
		}
		format!("<path d=\"M {} C {} {} {}\" {} {}/>", 
			(style.position + self.points[0]).svg(),
			(style.position + self.points[1]).svg(),
			(style.position + self.points[2]).svg(), 
			(style.position + self.points[3]).svg(),
			style.svg_basic_settings(),
			z
			)
	}

	fn get_area(&self, style: &Style) -> Area {
		let (mut min_x, mut max_x) = if self.points[0].x < self.points[3].x  {
			(self.points[0].x, self.points[3].x)
		} else {
			(self.points[3].x, self.points[0].x)
		};
		let (mut min_y, mut max_y) = if self.points[0].y < self.points[3].y {
			(self.points[0].y, self.points[3].y )
		} else {
			(self.points[3].y, self.points[0].y)
		};

		cubic_for_each_local_extremum(
			self.points[0].x,
			self.points[1].x,
			self.points[2].x,
			self.points[3].x,
			&mut |t| {
				let x = self.sample(t).x;
				if x < min_x {
					min_x = x;
				}
				if x > max_x {
					max_x = x;
				}
			},
		);

		cubic_for_each_local_extremum(
			self.points[0].y,
			self.points[1].y,
			self.points[2].y,
			self.points[3].y,
			&mut |t| {
				let y = self.sample(t).y;
				if y < min_y {
					min_y = y;
				}
				if y > max_y {
					max_y = y;
				}
			},
		);

		Area::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y)).transform(style)
	}

	fn delta(&self, rhs: &Self) -> Self {
		Self {
			points: [self.points[0] - rhs.points[0],
				self.points[1] - rhs.points[1],
				self.points[2] - rhs.points[2],
				self.points[3] - rhs.points[3]
			],
			..self.clone()
		}
	}

	fn change(&mut self, rhs: &Self) {
		*self = Self {
			points: [self.points[0] + rhs.points[0],
				self.points[1] + rhs.points[1],
				self.points[2] + rhs.points[2],
				self.points[3] + rhs.points[3]
			],
			..rhs.clone()
		}
	}

	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}



impl Animate for CubicBezier {
	fn animate(&mut self, id: usize, change_to: f32) {
		match id {
			0 => self.points[0].x = change_to,
			1 => self.points[0].y = change_to,
			2 => self.points[1].x = change_to,
			3 => self.points[1].y = change_to,
			4 => self.points[2].x = change_to,
			5 => self.points[2].y = change_to,
			6 => self.points[3].x = change_to,
			7 => self.points[3].y = change_to,
			_ => {},
		}
	}

	fn animate_len(&self) -> usize { 8 }
}

fn cubic_for_each_local_extremum<F: FnMut(f32)>(p0: f32, p1: f32, p2: f32, p3: f32, cb: &mut F) {
	let a = 3.0 * (p3 + 3.0 * (p1 - p2) - p0);
	let b = 6.0 * (p2 - 2.0 * p1 + p0);
	let c = 3.0 * (p1 - p0);

	let in_range = |t: f32| t <= 1.0 && t >= 0.0;

	if a == 0.0 {
		if b != 0.0 {
			let t = -c / b;
			if in_range(t) {
				cb(t);
			}
		}
		return;
	}

	let discr = b * b - 4.0 * a * c;
	if discr < 0.0 {
		return;
	}

	if discr == 0.0 {
		let t = -b / (2.0 * a);
		if in_range(t) {
			cb(t);
		}
		return;
	}

	let discr = discr.sqrt();
	let t1 = (-b - discr) / (2.0 * a);
	let t2 = (-b + discr) / (2.0 * a);
	if in_range(t1) {
		cb(t1);
	}
	if in_range(t2) {
		cb(t2);
	}
}

/// standard svg code.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Svg{
	/// svg code
	code: String,
	/// svg size, since shapoist didn't know how large your svg is.
	size: [Vec2; 2]
}

impl Shape for Svg {
	fn into_svg(&self, _: &Style) -> std::string::String { self.code.clone() }
	fn get_area(&self, _: &Style) -> Area { self.size.into() }
	fn delta(&self, _: &Self) -> Self { self.clone() }
	fn change(&mut self, _: &Self) {}
	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Animate for Svg {}

/// a image
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Image {
	/// image id
	pub id: String,
	/// image size
	pub size: Vec2,
	/// image mask, only inside will show
	pub mask: Option<ShapeMask>
}

impl Shape for Image {	
	fn into_svg(&self, _: &Style) -> std::string::String { todo!() }
	fn get_area(&self, style: &Style) -> Area { [style.position, style.position+self.size].into() }
	fn delta(&self, _: &Self) -> Self { todo!() }
	fn change(&mut self, _: &Self) { todo!() }
	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Animate for Image {}

/// useful for drawing gradient color or other complex shapes. should be sorted by counterclockwise.
///
/// # Panics
/// when the amount of vertex in [`Vertexs`] is less than 3
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Vertexs {
	pub vertexs: Vec<Vertex>,
	pub indices: Vec<[usize; 3]>,
}

impl Into<(Vec<Vertex>, Vec<[usize; 3]>)> for Vertexs {
	fn into(self) -> (Vec<Vertex>, Vec<[usize; 3]>) {
		(self.vertexs, self.indices)
	}
}

/// should be sorted by counterclockwise. only convex polygon can be filled correctly.
///
/// you can sort points by [`Self::sort()`]
///
/// # Panics
/// when the amount of vertex in a [`Polygon`] is less than 3
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Polygon {
	pub points: Vec<Vec2>,
	pub(crate) is_styled: bool,
}

impl Into<Polygon> for Vec<Vec2> {
	fn into(self) -> Polygon {
		Polygon {
			points: self,
			..Default::default()
		}
	}
}

impl Animate for Polygon {
	fn animate(&mut self, id: usize, change_to: f32) {
		if let Some(point) = self.points.get_mut(id / 2) {
			if id % 2 == 0 {
				point.x = change_to
			}else {
				point.y = change_to
			}
		}
	}
	fn animate_len(&self) -> usize {
		self.points.len() * 2
	}
}

impl Shape for Polygon {	
	fn into_svg(&self, style: &Style) -> String {
		let mut points = String::new();
		for point in &self.points {
			points = format!("{points} {}", (*point + style.position).svg())
		}

		format!("<polygon  points=\"{}\" {} />", 
			points,
			style.svg_basic_settings()
			) 
	}
	fn get_area(&self, style: &Style) -> Area {
		// stupid
		let mut min = Vec2::same(f32::INFINITY);
		let mut max = Vec2::same(f32::NEG_INFINITY);
		for point in &self.points {
			if point.x < min.x {
				min.x = point.x
			}else if point.x > max.x {
				max.x = point.x
			}
			if point.y < min.y {
				min.y = point.y
			}else if point.y > max.y {
				max.y = point.y
			}
		}
		Area::new(min, max).transform(style)
	}
	fn delta(&self, _: &Self) -> Self { todo!() }
	fn change(&mut self, _: &Self) { todo!() }
	#[cfg(feature = "vertexs")]
	fn into_vertexs(&self, _: &Style, _: Vec2) -> (Vec<Vertex>, Vec<u32>) { todo!() }
}

impl Polygon {
	/// push a point into a polygon
	pub fn push(&mut self, point: Vec2) {
		self.points.push(point);
	}

	/// how much points do we have in this [`Polygon`]?
	pub fn len(&self) -> usize {
		self.points.len()
	}

	/// append a polygon into another
	pub fn append(&mut self, other: &mut Self) {
		self.points.append(&mut other.points);
	}

	/// append a polygon into another with some process on other one
	pub fn append_with_process(&mut self, other: &mut Self, mut process: impl FnMut(Vec2) -> Vec2) {
		for point in &mut other.points {
			*point = process(*point)
		}
		self.append(other)
	}

	/// create a new polygon
	pub fn new() -> Self {
		Self {
			points: vec!(),
			..Default::default()
		}
	}

	/// check if a point is on this polygon
	pub fn contains(&self, point: &Vec2) -> bool {
		self.points.contains(point)
	}

	/// check if a point is inside this polygon.
	pub fn is_point_inside(&self, point: Vec2) -> bool {
		let mut result = false; 
		if self.contains(&point) {
			return true
		}
		for point_id in 0..self.points.len() {
			let id1 = point_id;
			let id2 = (point_id + 1) % self.points.len();
			if point.is_point_on_line(&self[id1], &self[id2]) {
				return true
			}
			if (self[id1].y - point.y > 0.0) != (self[id2].y - point.y > 0.0) {
				let vt = (point.y - self[id1].y) / (self[id2].y - self[id1].y);
				if point.x < self[id1].x + vt * (self[id2].x - self[id1].x) {
					result = !result
				}
			}
		}

		result
	}

	/// move a polygon to a new place
	pub fn move_to(&mut self, position: Vec2) {
		for point in &mut self.points {
			*point = *point + position;
		}
	}

	/// get the center of this [`Polygon`]
	pub fn center(&self) -> Vec2 {
		let center = self.points.par_iter().cloned().reduce(|| Vec2::ZERO, |a, b| a + b) / (self.len() as f32);
		center
	}

	/// sort a [`Polygon`]'s points by
	///
	/// # Panics
	/// when have [`f32::NAN`]
	pub fn sort(&mut self) {
		let center = self.center();
		self.points.par_sort_by(|a,b| (*b - center).angle().partial_cmp(&(*a - center).angle()).unwrap());
	}
}

impl Index<usize> for Polygon {
	type Output = Vec2;
	fn index(&self, index: usize) -> &Vec2 {
		&self.points[index]
	}
}

impl IntoIterator for Polygon {
	type Item = Vec2;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.points.into_iter()
	}
}

fn compress(input: f32) -> f32 {
	if input < 0.0 {
		return 0.0;
	}else if input > 1.0 {
		return 1.0;
	}
	input
}