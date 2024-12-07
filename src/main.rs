mod moose;
mod drone;
mod utils;
use moose::{Area, Moose};
use rand::Rng;
use std::io;

fn main() {
    let paramsets: Vec<String> = io::stdin()
        .lines()
        .map(|x| x.expect("read error"))
        .collect();
    for params in paramsets {
        let params: Vec<&str> = params.split_whitespace().collect();
        // moose amount, moose speed, moose eat speed, drone speed, drone range, drone path loops

        let mut animals: Vec<Moose> = vec![];
        let mut feeding_areas: Vec<Area> = vec![];
        let mut sleeping_areas: Vec<Area> = vec![];
        let mut rng = rand::thread_rng();
        let mut drone = drone::drone((500, 500), 500, 10, 2);

        for _ in 0..rng.gen_range(10..20) {
            feeding_areas.push(Area { center: (rng.gen_range(0..20000), rng.gen_range(0..11000)), radius: rng.gen_range(0..1000)});
        }

        for _ in 0..rng.gen_range(10..20) {
            sleeping_areas.push(Area { center: (rng.gen_range(0..20000), rng.gen_range(0..11000)), radius: rng.gen_range(0..1000)});
        }

        for _ in 0..26_000_000 {
            let pos = (rng.gen_range(0..20000), rng.gen_range(0..11000));
            animals.push(moose::moose(pos, 4.0, 2.0, 3, 3));
        }

        let mut counted_moose = 0;
        loop {
            for mut moose in animals {
                moose.timestep(&feeding_areas, &sleeping_areas);
            }
            let (timestep_moose, finished) = drone.timestep(&animals);
            if finished {
                break;
            } else {
                counted_moose += timestep_moose;
            }
        }
    }
}
