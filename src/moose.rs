
struct Area {
    center: (i32, i32),
    radius: i32,
    is_rest_area: bool
}

pub struct Moose {
    pub pos: (i32, i32),
    pub speed: f64,
    pub target: (i32, i32),
    pub rest_area_id: i32,
}

impl Moose {
    fn move_to_target(&mut self) {
        let vector: (f64, f64) = ((self.target.0 - self.pos.0) as f64, (self.target.1 - self.pos.1) as f64);
        let vector_length = (vector.0 + vector.1).sqrt();
        let dir_vector = (vector.0 /vector_length, vector.1/vector_length);
        self.pos = (self.pos.0 + (dir_vector.0*self.speed).round() as i32, self.pos.1 + (dir_vector.1*self.speed).round() as i32) 
    }
}
pub struct MooseChild {
    pub pos: (i32, i32),
    speed: i32,
    pub parent_id: i32
}

