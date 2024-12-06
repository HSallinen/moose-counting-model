struct Moose {
    id: i32,
    cordinates: (i32, i32),
    rest_center: (i32, i32, i32),
    feeding_areas: Vec<(i32, i32, i32)>,
}

struct MooseChild {
    id: i32,
    cordinates: (i32, i32),
    parent_id: i32,
}
