use crate::model::Drawing;
use std::io::Error;

pub trait Parser {
    fn parse(self) -> Result<Drawing, Error>;
}

pub mod svg;
