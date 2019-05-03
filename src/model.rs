use lyon_path::Path;


pub struct Drawing {
    paths: Vec<DrawingPath>
}

pub struct DrawingPath{
    pub color: u8,
    pub path: Path
}

impl Drawing{
    pub fn new() -> Self {
        Self{
            paths: Vec::new()
        }
    }

    pub fn add_path(&mut self, path: Path, color: u8){
        self.paths.push(DrawingPath{
            color,
            path
        });
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, DrawingPath>{
        self.paths.iter()
    }
}
impl DrawingPath {
    pub fn color(&self) -> u8 {
        self.color
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
