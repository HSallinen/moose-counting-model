use crate::moose;
use crate::utils::distance_squared;

pub struct Drone {
    pub pos: (i32, i32),
    detection_range: i32,
    speed: i32,
    last_detected: Vec<usize>,
    path: Vec<(i32, i32)>,
    path_target_index: usize,
}

pub fn drone(pos: (i32, i32), detection_range: i32, speed: i32, path: Vec<(i32, i32)>) -> Drone {
    Drone {
        pos,
        detection_range,
        speed,
        last_detected: vec![],
        path,
        path_target_index: 0,
    }
}

impl Drone {
    fn count_moose(&mut self, moose: Vec<moose::Moose>) -> i32 {
        let mut acc = 0;
        let mut found: Vec<usize> = vec![];

        for (index, moose) in moose.iter().enumerate() {
            if distance_squared(moose.pos, self.pos) < self.detection_range.pow(2) {
                if !self.last_detected.contains(&index) {
                    acc += 1;
                }
                found.push(index);
            }
        }
        self.last_detected = found;
        acc
    }

    fn timestep(&mut self, moose: Vec<moose::Moose>) -> i32 {
        if distance_squared(self.path[self.path_target_index], self.pos) < 100 {
            self.path_target_index += 1;
        }
        let vector: (f64, f64) = ((self.path[self.path_target_index].0 - self.pos.0) as f64,
                                  (self.path[self.path_target_index].1 - self.pos.1) as f64);
        let vector_length = (vector.0 + vector.1).sqrt();
        let dir_vector = (vector.0 / vector_length, vector.1 / vector_length);
        self.pos = (self.pos.0 + (dir_vector.0 * self.speed as f64).round() as i32, self.pos.1 + (dir_vector.1 * self.speed as f64).round() as i32);
        self.count_moose(moose)
    }
}
