use std::io::Error;
use crate::model::Drawing;

pub trait Exporter{
    fn export(self, drawing: Drawing) -> Result<(), Error>;
}

pub mod gcode;
