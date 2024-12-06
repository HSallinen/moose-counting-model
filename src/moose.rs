pub struct Moose {
    pub pos: (i32, i32),
    pub rest_area: ((i32, i32), i32),
    pub feeding_areas: Vec<((i32, i32), i32)>,
}

pub struct MooseChild {
    pub pos: (i32, i32),
    pub parent_id: i32,
}
