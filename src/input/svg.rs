use usvg::Color;
use super::Parser;
use crate::model::Drawing;
use lyon_path::Path;
use lyon_path::math::{Point, point};
use std::path::Path as FilePath;
use std::io::{Error, ErrorKind};
use usvg::{Paint, PathSegment};

pub const FALLBACK_COLOR: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
};

pub struct SvgParser<'a> {
    input: &'a FilePath,
}

impl <'a> SvgParser<'a> {
    pub fn new(path: &'a FilePath) -> Self {
        Self{
            input: path
        }
    }
}

impl <'a> Parser for SvgParser<'a> {
    fn parse(mut self) -> Result<Drawing, Error> {
        let opt = usvg::Options::default();

        let tree = usvg::Tree::from_file(&self.input, &opt).or_else(|e|{
            Err(Error::new(ErrorKind::Other, "USVG error"))
        })?;

        let mut drawing = Drawing::new();

        for node in tree.root().descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {

                let mut builder = Path::builder();

                for ref segment in &p.segments {
                    match segment{
                        PathSegment::MoveTo{x, y} => {
                            builder.move_to(point(*x as f32, *y as f32));
                        },
                        PathSegment::LineTo{x, y} => {
                            builder.line_to(point(*x as f32, *y as f32));
                        },
                        PathSegment::CurveTo { x1, y1, x2, y2, x, y } => {
                            builder.cubic_bezier_to(
                                point(*x1 as f32, *y1 as f32),
                                point(*x2 as f32, *y2 as f32),
                                point(*x as f32, *y as f32)
                            )
                        },
                        PathSegment::ClosePath => {
                            builder.close()
                        }
                    }
                }

                drawing.add_path(builder.build(), 0);
            }
        }

        Ok(drawing)
    }
}
