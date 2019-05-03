use lyon_path::{Path, FlattenedEvent, iterator::PathIterator};
use std::io::Error;
use std::fs::File;
use std::io::{Write, BufWriter};
use super::Exporter;
use crate::model::{DrawingPath, Drawing};
use lyon_path::math::{Point, point, Vector, vector};
use lyon_path::geom::euclid::Transform2D;
use std::path::Path as FilePath;

pub struct GcodeExporter<'a> {
    output: &'a FilePath,
    threshold: f32,
    zhop: f32,
    zfeedrate: f32,
    drawfeedrate: f32,
    movefeedrate: f32,
    ysize: f32
}

impl <'a> GcodeExporter<'a> {
    pub fn new(path: &'a FilePath) -> Self {
        Self{
            output: path,
            threshold: 0.1,
            zhop: 1.0,
            zfeedrate: 300.0 * 60.0,
            drawfeedrate: 60.0 * 60.0,
            movefeedrate: 120.0 * 60.0,
            ysize: 200.0
        }
    }

    pub fn with_threshold(mut self, t: f32) -> Self {
        self.threshold = t;
        self
    }

    /// Sets Z-hop, which is the height for travelling movements
    pub fn with_zhop(mut self, z: f32) -> Self {
        self.zhop = z;
        self
    }

    /// Sets the speed of Z-hops. In mm/s
    pub fn with_zfeedrate(mut self, z: f32) -> Self {
        self.zfeedrate = z * 60.0;
        self
    }

    /// Sets the speed of drawing movements. In mm/s
    pub fn with_draw_feedrate(mut self, fr: f32) -> Self {
        self.drawfeedrate = fr * 60.0;
        self
    }

    /// Sets the speed of travelling. In mm/s
    pub fn with_move_feedrate(mut self, fr: f32) -> Self {
        self.movefeedrate = fr * 60.0;
        self
    }

    /// Sets bed Y size
    pub fn with_y_size(mut self, y: f32) -> Self {
        self.ysize = y;
        self
    }
}

impl <'a> Exporter for GcodeExporter<'a> {
    fn export(self, drawing: Drawing) -> Result<(), Error>{
        let file = File::create(self.output)?;
        let mut gcode = BufWriter::new(file);

        writeln!(gcode, ";SVGCODE generated this shit!")?;
        writeln!(gcode, "G28; Autohome")?;

        let transform = Transform2D::create_scale(1.0, -1.0).post_translate(vector(0.0, self.ysize));
        for DrawingPath{color: _, path} in drawing.iter(){

            for event in path.iter().transformed(&transform).flattened(self.threshold){
                match event{
                    FlattenedEvent::MoveTo(p) => {
                        writeln!(gcode, "G0 F{} Z{}", self.zfeedrate, self.zhop)?;
                        writeln!(gcode, "G0 F{} X{} Y{}", self.movefeedrate, p.x, p.y)?;
                        writeln!(gcode, "G0 F{} Z0", self.zfeedrate)?;
                    },
                    FlattenedEvent::Line(s) => {
                        writeln!(gcode, "G1 F{} X{} Y{}", self.drawfeedrate, s.to.x, s.to.y)?;
                    },
                    FlattenedEvent::Close(s) => {
                        writeln!(gcode, "G1 F{} X{} Y{}", self.drawfeedrate, s.to.x, s.to.y)?;
                    }
                }
            }
        }

        writeln!(gcode, "G0 F{} Z5", self.zfeedrate)?;
        gcode.flush()?;

        Ok(())
    }
}
