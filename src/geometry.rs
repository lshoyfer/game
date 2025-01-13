use crate::prelude::*;

pub mod map_parser;
pub use map_parser::*;

#[derive(Debug)]
pub struct Line {
    pub inner: Vec4,
}

#[derive(Debug)]
pub struct Geometry {
    pub kind: GeometryType
}

impl Geometry {
    pub fn new_line(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Geometry { kind: GeometryType::Line(Line { inner: vec4(x1, y1, x2 ,y2) }) }
    }

    pub fn new_rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Geometry { kind: GeometryType::Rect(Rect::new(x, y, w, h)) }
    }
}

#[derive(Debug)]
pub enum GeometryType {
    Rect(Rect),
    Line(Line),
    // is_wholly_inside_a_chunk: bool,
}

// impl RectBounded for Geometry {
//     fn ref_boundary(&self) -> &Rect {
//         &self.boundary
//     }

//     fn mut_boundary(&mut self) -> &mut Rect {
//         &mut self.boundary
//     }
// }

/* // REVIEW
/// 256 x 256 logical pixels, contains a list of geometry that are either wholly or partially inside of it
pub struct Chunk {
    pub inner: Vec<Geometry>,
}

pub struct GeometryMap {
    pub chunks: Vec<Chunk>
}
*/

#[derive(Debug)]
pub struct GeometryMap {
    pub inner: Vec<Geometry>
}