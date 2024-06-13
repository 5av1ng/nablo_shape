//! Provid some basic abstraction of shapes.
pub mod shape_elements;
pub mod animation;

use std::ops::IndexMut;
use std::ops::Index;
use crate::math::Area;
use crate::math::Vec2;
use crate::shape::shape_elements::Shape as ShapeTrait;
use crate::shape::shape_elements::*;
use crate::shape::shape_elements::Style;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Default)]
#[serde(default)]
/// a collection of shape.
pub struct Shape {
	pub style: Style,
	pub shape: ShapeElement,
}

/// the way you paint stuff in `nablo`
///
/// you can use index value to get a drawn shape and edit it.
///
/// # Example 
/// ```
/// # use nablo_shape::prelude::Vec2;
/// # use nablo_shape::prelude::ShapeElement;
/// # use nablo_shape::prelude::shape_elements::Rect;
/// # use nablo_shape::prelude::shape_elements::Style;
/// # use nablo_shape::prelude::Painter;
/// # use nablo_shape::prelude::Area;
/// # use nablo_shape::prelude::Shape;
/// let mut painter = Painter::from_area(&Area::new_with_origin(Vec2::same(123.0)));
///	painter.rect(Vec2::same(100.0), Vec2::ZERO);
/// assert_eq!(painter[0], Shape {
///     style: Style {
///         clip: Area::new_with_origin(Vec2::same(123.0)),
///         ..Default::default()
///     },
///     shape: ShapeElement::Rect(Rect{
///         width_and_height: Vec2::same(100.0),
///         rounding: Vec2::ZERO,
///     })
/// })
/// ```
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Painter {
	pub paint_area: Area,
	pub offset: Vec2,
	shapes: Vec<Shape>,
	style: Style,
	text_style: TextStyle
}

impl Index<usize> for Painter {
	type Output = Shape;
	fn index(&self, index: usize) -> &Self::Output { 
		&self.shapes[index]
	}
}

impl IndexMut<usize> for Painter {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { 
		&mut self.shapes[index]
	}
}

impl Default for Painter {
	fn default() -> Self {
		Self {
			shapes: vec!(),
			style: Style::default(),
			paint_area: Area::default(),
			offset: Vec2::same(0.0),
			text_style: TextStyle::default()
		}
	}
}

impl Into<Vec<Shape>> for Painter {
	fn into(self) -> Vec<Shape> { self.shapes }
}

impl Painter {
	/// set where should we draw, Note: this function only affects on newly added shapes.
	pub fn set_position(&mut self, position: Vec2) {
		self.style.position = position + self.offset;
	}

	/// set the transform origin, Note: this function only affects on newly added shapes.
	pub fn set_transform_origin(&mut self, transform_origin: Vec2) {
		self.style.transform_origin = transform_origin;
	}

	/// rotate a shape, Note: this function only affects on newly added shapes. follows radian measure
	pub fn set_rotate(&mut self, rotate: f32) {
		self.style.rotate = rotate;
	}

	/// scale a shape, Note: this function only affects on newly added shapes. 
	pub fn set_scale(&mut self, size: Vec2) {
		self.style.size = size;
	}

	/// set color of a shape, Note: this function only affects on newly added shapes.
	pub fn set_color(&mut self, fill: impl Into<Color>) {
		self.style.fill = fill.into();
	}

	/// set stroke width of a shape, Note: this function only affects on newly added shapes.
	pub fn set_stroke_width(&mut self, width: f32) {
		self.style.stroke_width = width;
	}

	/// set stroke color of a shape, Note: this function only affects on newly added shapes.
	pub fn set_stroke_color(&mut self, color: impl Into<Color>) {
		self.style.stroke_color = color.into();
	}

	/// set which layer will we paint, Note: this function only affects on newly added shapes.
	pub fn set_layer(&mut self, layer: Layer) {
		self.style.layer = layer;
	}

	/// set offset of this painter, normally you would not use this unless you are creating a container or a canvas or etc.
	pub fn set_offset(&mut self, offset: Vec2) {
		self.offset = offset;
		self.style.position = self.style.position + self.offset;
	}

	/// set clip color of a shape, Note: this function only affects on newly added shapes.
	pub fn set_clip(&mut self, clip: Area) {
		self.style.clip = clip
	}

	/// set text to paint bold or not
	pub fn set_text_bold(&mut self, is_bold: bool) {
		self.text_style = self.text_style.clone().set_bold(is_bold);
	}

	/// set text to paint bold or not
	pub fn set_text_italic(&mut self, is_italic: bool) {
		self.text_style = self.text_style.clone().set_italic(is_italic);
	}

	/// set text style dirctly
	pub fn set_text_style(&mut self, text_style: TextStyle) {
		self.text_style = text_style;
	}

	/// draw a given shape
	///
	/// note: will return [`Option::None`] if the shape is not paint(eg. outside the draw rect or cant visible)
	pub fn draw(&mut self, shape: ShapeElement) -> Option<usize>  {
		let shape = Shape {
			style: self.style.clone(),
			shape,
			..Default::default()
		};

		if (shape.style.fill[3] == 0) && (shape.style.stroke_color[3] == 0 || shape.style.stroke_width == 0.0) {
			return None
		}

		if self.paint_area.is_inside(&shape.get_area()) || self.paint_area.is_cross(&shape.get_area()) || shape.get_area().is_inside(&self.paint_area) {
			self.shapes.push(shape);
		}else {
			return None
		}

		let len = self.shapes.len();
		Some(len - 1)
	}

	/// draw a line. see more in [`Self::draw`]
	pub fn line(&mut self, other_point: Vec2) -> Option<usize> {
		self.draw(ShapeElement::Line(other_point))
	}

	/// draw a rectangle. see more in [`Self::draw`]
	pub fn rect(&mut self, width_and_height: Vec2, rounding: Vec2) -> Option<usize> {
		self.draw(ShapeElement::Rect(Rect {
			width_and_height,
			rounding,
		}))
	}

	/// draw a circle. see more in [`Self::draw`]
	pub fn cir(&mut self, radius: f32) -> Option<usize> {
		self.draw(ShapeElement::Circle(Circle {
			radius
		}))
	}

	/// draw a cubic bezier curve. see more in [`Self::draw`]
	pub fn bezier(&mut self, points: [Vec2; 4]) -> Option<usize> {
		self.draw(ShapeElement::CubicBezier(CubicBezier {
			points,
			if_close: false,
		}))
	}

	/// draw a polygon. see more in [`Self::draw`] 
	pub fn polygon(&mut self, points: Vec<Vec2>) -> Option<usize> {
		self.draw(ShapeElement::Polygon(
			points.into()
		))
	}

	/// draw a image. see more in [`Self::draw`]
	pub fn image(&mut self, id: impl Into<String>, size: Vec2) -> Option<usize> {
		let id = id.into();
		self.draw(ShapeElement::Image(Image {
			id,
			size,
			mask: None
		}))
	}

	/// draw a image with mask. see more in [`Self::draw`]
	pub fn image_mask(&mut self, id: impl Into<String>, size: Vec2, mask: ShapeMask) -> Option<usize> {
		let id = id.into();
		self.draw(ShapeElement::Image(Image {
			id,
			size,
			mask: Some(mask)
		}))
	}

	/// draw a text
	pub fn text(&mut self, text: String) -> Option<usize> {
		self.draw(ShapeElement::Text(Text {
			text,
			text_height: None,
			text_width: None,
			text_style: self.text_style.clone(),
			..Default::default()
		}))
	}

	/// get how large do a text take without break.
	pub fn text_area(&self, text: String) -> Area {
		let text = Shape {
			style: self.style.clone(),
			shape: ShapeElement::Text(Text {
				text,
				text_height: None,
				text_width: None,
				text_style: self.text_style.clone(),
				..Default::default()
			}),
			..Default::default()
		};
		text.get_area()
	}

	/// get how large do a text take with break.
	pub fn text_area_width(&self, text: String, width: f32) -> Area {
		let text = Shape {
			style: self.style.clone(),
			shape: ShapeElement::Text(Text {
				text,
				text_height: None,
				text_width: Some(width),
				text_style: self.text_style.clone(),
				..Default::default()
			}),
			..Default::default()
		};
		text.get_area()
	}

	/// draw a text with width limit. see more in [`Self::draw`]
	pub fn text_with_width(&mut self, text: String, width: f32) -> Option<usize> {
		self.draw(ShapeElement::Text(Text {
			text,
			text_height: None,
			text_width: Some(width),
			text_style: self.text_style.clone(),
			..Default::default()
		}))
	}

	/// draw a text with limit. see more in [`Self::draw`]
	pub fn text_with_limit(&mut self, text: String, width: f32, height: f32) -> Option<usize> {
		self.draw(ShapeElement::Text(Text {
			text,
			text_height: Some(height),
			text_width: Some(width),
			text_style: self.text_style.clone(),
			..Default::default()
		}))
	}

	/// get a painter from area, anything outside of this area would not showed on screen.
	pub fn from_area(paint_area: &Area) -> Self {
		let back = Self {
			paint_area: *paint_area,
			style: Style {
				clip: *paint_area, 
				..Default::default()
			}, 
			..Default::default()
		};
		back
	}

	/// create a new paniter.
	pub fn new(paint_area: &Area, shapes: Vec<Shape>,style: Style) -> Self {
		let back = Self {
			paint_area: *paint_area,
			shapes,
			style: Style {
				clip: *paint_area, 
				..style
			}, 
			..Default::default()
		};
		back
	}

	/// move every drawn shapes to a new place, also changes position and transform_origin
	pub fn move_delta_to(&mut self, delta: Vec2){
		self.paint_area.move_delta_to(delta);
		self.style.clip.move_delta_to(delta);
		self.style.position = self.style.position + delta;
		self.style.transform_origin = self.style.transform_origin + delta;
		for shape in &mut self.shapes {
			shape.move_delta_to(delta)
		}
	}

	/// change drawn shapes' color, let them become brighter or darker
	pub fn brighter(&mut self, factor: f32){
		for shape in &mut self.shapes {
			shape.style.fill = shape.style.fill.brighter(factor);
			shape.style.stroke_color = shape.style.stroke_color.brighter(factor)
		}
	}

	/// change drawn shapes' clip.
	pub fn change_clip(&mut self, clip: Area) {
		for shape in &mut self.shapes {
			shape.style.clip = clip;
		}
	}

	/// change drawn shapes' rotate.
	pub fn change_rotate(&mut self, rotate: f32) {
		for shape in &mut self.shapes {
			shape.style.rotate = rotate;
		}
	}

	/// change drawn shapes' transform_origin.
	pub fn change_transform_origin(&mut self, transform_origin: Vec2) {
		for shape in &mut self.shapes {
			shape.style.transform_origin = transform_origin;
		}
	}

	/// change drawn shapes' layer, also affect shapes to be drawn
	pub fn change_layer(&mut self, layer: Layer){
		self.set_layer(layer);
		for shape in &mut self.shapes {
			shape.style.layer = layer;
		}
	}

	/// check is any shape painted in this [`Painter`]
	pub fn is_empty(&self) -> bool {
		self.shapes.is_empty()
	}

	/// append another [`Painter`] to current [`Painter`], will not recut.
	pub fn append(&mut self, other: &mut Painter) {
		self.shapes.append(&mut other.shapes)
	}

	/// push a shape into current painter
	pub fn push(&mut self, shape: Shape) {
		self.shapes.push(shape)
	}

	/// get how large place did current shapes take
	pub fn get_area(&self) -> Area {
		let mut area = Area::ZERO;
		for shape in &self.shapes {
			let rect = shape.get_area();
			area.combine(&rect);
		}
		area
	}

	/// get the style on this [`Painter`]
	pub fn style(&self) -> &Style {
		&self.style
	}

	/// get the style on this [`Painter`], but muttable
	pub fn style_mut(&mut self) -> &mut Style {
		&mut self.style
	}

	/// get the text style on this [`Painter`]
	pub fn text_style(&self) -> &TextStyle {
		&self.text_style
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
/// what shape is us?
pub enum ShapeElement {
	Circle(Circle),
	Rect(Rect),
	Text(Text),
	CubicBezier(CubicBezier),
	Line(Vec2),
	Polygon(Polygon),
	Image(Image)
}

impl ShapeElement {
	/// turn current [`ShapeElement`] into [`ShapeMask`]
	///
	/// # Panics
	/// when meets `ShapeElement::Image(_)` or `ShapeElement::Text(_)` 
	pub fn into_mask(&self) -> ShapeMask {
		match self {
			Self::Circle(t) => ShapeMask::Circle(t.clone()),
			Self::Rect(t) => ShapeMask::Rect(t.clone()),
			Self::CubicBezier(t) => ShapeMask::CubicBezier(t.clone()),
			Self::Polygon(t) => ShapeMask::Polygon(t.clone()),
			Self::Line(t) => ShapeMask::Line(t.clone()),
			_ => unreachable!()
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
/// for rust checker
pub enum ShapeMask {
	Circle(Circle),
	Rect(Rect),
	Line(Vec2),
	CubicBezier(CubicBezier),
	Polygon(Polygon),
}

impl Default for ShapeElement {
	fn default() -> Self {
		Self::Circle(Circle::default())
	}
}

impl ShapeMask {
	/// convert a shape into vertexs
	#[cfg(feature = "vertexs")]
	pub fn into_vertexs(&self, size: Vec2, style: &Style) -> (Vec<Vertex>, Vec<u32>, Area) {
		match &self {
			ShapeMask::Circle(t) => {
				t.into_vertexs(style, size)
			},
			ShapeMask::Rect(t) => {
				t.into_vertexs(style, size)
			},
			ShapeMask::CubicBezier(t) => {
				t.into_vertexs(style, size)
			},
			ShapeMask::Polygon(t) => t.into_vertexs(style, size),
			ShapeMask::Line(t) => t.into_vertexs(style, size),
		}
	}
}

impl Shape {
	/// convert a shape into vertexs
	#[cfg(feature = "vertexs")]
	pub fn into_vertexs(&self, size: Vec2) -> (Vec<Vertex>, Vec<u32>, Area) {
		match &self.shape {
			ShapeElement::Circle(t) => {
				t.into_vertexs(&self.style, size)
			},
			ShapeElement::Rect(t) => {
				t.into_vertexs(&self.style, size)
			},
			ShapeElement::CubicBezier(t) => {
				t.into_vertexs(&self.style, size)
			},
			ShapeElement::Polygon(t) => t.into_vertexs(&self.style, size),
			ShapeElement::Text(_) => unreachable!(),
			ShapeElement::Image(_) => unreachable!(),
			ShapeElement::Line(t) => t.into_vertexs(&self.style, size),
		}
	}

	/// pre-scale current shape with (0,0), useful when scaling svg.
	pub fn pre_scale(&mut self, scale_factor: f32) {
		self.style = Style {
			position: self.style.position * scale_factor,
			transform_origin: self.style.transform_origin * scale_factor,
			stroke_width: self.style.stroke_width * scale_factor,
			clip: [self.style.clip.area[0] * scale_factor, self.style.clip.area[1] * scale_factor].into(),
			..self.style
		};
		match &mut self.shape {
			ShapeElement::Circle(t) => {
				t.radius = t.radius * scale_factor;
			},
			ShapeElement::Rect(t) => {
				t.width_and_height = t.width_and_height * scale_factor;
				t.rounding = t.rounding * scale_factor;
			},
			ShapeElement::CubicBezier(t) => {
				for point in &mut t.points {
					*point = *point * scale_factor;
				}
			},
			ShapeElement::Polygon(t) => {
				for point in &mut t.points {
					*point = *point * scale_factor;
				}
			},
			ShapeElement::Text(_) => {
				// not a good idea
				self.style.size = self.style.size * scale_factor;
			},
			ShapeElement::Image(t) => {
				t.size = t.size * scale_factor
			},
			ShapeElement::Line(t1) => {
				*t1 = *t1 * scale_factor;
			},
		}
	}

	/// get how much place this shape take
	pub fn get_area(&self) -> Area {
		match &self.shape {
			ShapeElement::Circle(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::Rect(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::CubicBezier(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::Text(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::Polygon(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::Image(t) => {
				t.get_area(&self.style)
			},
			ShapeElement::Line(t1) => {
				Area::new(self.style.position, *t1).transform(&self.style)
			},
		}
	}

	/// move a shape to a new place
	pub fn move_delta_to(&mut self, delta: Vec2) {
		self.style.clip.move_delta_to(delta);
		self.style.transform_origin = self.style.transform_origin + delta;
		self.style.position = self.style.position + delta;
		if let ShapeElement::Polygon(t) = &mut self.shape {
			if t.is_styled {
				t.move_to(delta);
			}
		}
	}
}