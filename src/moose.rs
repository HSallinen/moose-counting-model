use rand::{thread_rng, Rng};
use rand::distributions::Open01;
use crate::utils::distance_squared;
pub struct Area {
    pub center: (i32, i32),
    pub radius: i32,
}

pub enum MooseState {
    Walking,
    Eating,
    Sleeping,
}
pub enum NextTarget {
    Eat,
    Sleep,
}
pub struct Moose {
    pub pos: (i32, i32),
    pub speed: f64,
    pub eat_walk_speed: f64,
    pub target: (i32, i32),
    pub eat_time: i32,
    pub sleep_time: i32,
    time_slept: i32,
    time_eaten: i32,
    current_state: MooseState,
    next_target: NextTarget,
}

pub fn moose(pos: (i32, i32), speed: f64, eat_walk_speed: f64, eat_time: i32, sleep_time: i32) -> Moose {
    Moose{pos, speed, eat_walk_speed, target:pos, eat_time, sleep_time, time_eaten: 0, time_slept: 0,current_state: MooseState::Walking, next_target: NextTarget::Eat}
}
impl Moose {
    fn move_to_target(&mut self) -> bool{
        let vector: (f64, f64) = ((self.target.0 - self.pos.0) as f64, (self.target.1 - self.pos.1) as f64);
        let vector_length: f64 = (vector.0.powi(2) + vector.1.powi(2)).sqrt();
        if vector_length < self.speed {
            self.pos = self.target;
            return true
        }
        else{
            let dir_vector: (f64, f64) = (vector.0 /vector_length, vector.1/vector_length);
            self.pos = (self.pos.0 + (dir_vector.0*self.speed).round() as i32, self.pos.1 + (dir_vector.1*self.speed).round() as i32) 
            return false;
        }
    }

    fn choose_target(&mut self, areas: &Vec<Area>) {
        let mut inverse_squares: Vec<f64> = Vec::with_capacity(areas.len());
        let mut inverse_sum: f64 = 0.0;
        for area in areas {
            let distance_squared: f64 = distance_squared(area.center, self.pos) as f64;
            let inverse_square: f64 = 1.0/distance_squared;
            inverse_squares.push(inverse_square);
            inverse_sum += inverse_square;
        }
        let mut chance_table: Vec<f64> = Vec::with_capacity(areas.len());
        let mut coefficient_sum: f64 = 0.0;
        for inverse_square in inverse_squares {
            let coefficient: f64 = inverse_square/inverse_sum;
            chance_table.push(coefficient_sum);
            coefficient_sum += coefficient;
        }
        let rand: f64 = thread_rng().sample(Open01);
        let index: usize = chance_table.partition_point(|lower| lower<&rand) -1;
        self.target = areas[index].center
    }

    fn random_move(&mut self, speed: f64) {
        self.pos.0 += thread_rng().gen_range(-speed..speed).round() as i32;
        self.pos.1 += thread_rng().gen_range(-speed..speed).round() as i32;
    }

    pub fn timestep(&mut self, eating_areas: &Vec<Area>, sleeping_areas: &Vec<Area>) {
        match self.current_state {
            MooseState::Walking => { 
                if self.move_to_target() {
                    match self.next_target {    
                        NextTarget::Eat => {
                            self.current_state = MooseState::Eating;
                            self.time_eaten = 0;
                        }
                        NextTarget::Sleep => {
                            self.current_state = MooseState::Sleeping;
                            self.time_slept = 0;
                        }
                    }
                }
            }
            MooseState::Eating => {
                self.time_eaten += 1;
                self.random_move(self.eat_walk_speed);
                if self.time_eaten >= self.eat_time {
                    self.current_state = MooseState::Walking;
                    self.next_target = NextTarget::Sleep;
                    self.choose_target(sleeping_areas);
                }
            }
            MooseState::Sleeping => {
                self.time_slept += 1;
                if self.time_slept >= self.sleep_time {
                    self.current_state = MooseState::Walking;
                    self.next_target = NextTarget::Eat;
                    self.choose_target(eating_areas);
                }
            }
        }
    }

}
pub struct MooseChild {
    pub pos: (i32, i32),
    speed: i32,
    pub parent_id: i32
}

