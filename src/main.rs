mod moose;
mod drone;
mod utils;
use moose::{Area, Moose};
use rand::Rng;

fn main() {
    let mut animals: Vec<Moose> = vec![];
    let mut feeding_areas: Vec<Area> = vec![];
    let mut rng = rand::thread_rng();
    let mut drone = drone::drone((500, 500), 500, 10, vec![(500, 10500), (1500, 10500), (1500, 500)]);

    for _ in 0..rng.gen_range(10..20) {
        feeding_areas.push(Area { center: (rng.gen_range(0..20000), rng.gen_range(0..11000)), radius: rng.gen_range(0..1000)});
    }

    for _ in 0..26_000_000 {
        let pos = (rng.gen_range(0..20000), rng.gen_range(0..11000));
        animals.push(moose::moose(pos, 4.0, 2.0));
    }

    let mut counted_moose = 0;
    loop {
        for mut moose in animals {
            moose.timestep();
        }
        let (timestep_moose, finished) = drone.timestep(&animals);
        if finished {
            break;
        } else {
            counted_moose += timestep_moose;
        }
    }
}
