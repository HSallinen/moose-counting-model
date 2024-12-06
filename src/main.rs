mod moose;
mod drone;
use moose::Moose;
use rand::Rng;

fn main() {
    let mut animals: Vec<Moose> = vec![];
    let mut feeding_areas: Vec<((i32, i32), i32)> = vec![];
    let mut rng = rand::thread_rng();
    let mut drone = drone::drone((500, 500), 500, 10, vec![(500, 10500), (1500, 10500), (1500, 500)]);

    for _ in 0..rng.gen_range(10..20) {
        feeding_areas.push(((rng.gen_range(0..20000), rng.gen_range(0..11000)), rng.gen_range(0..1000)));
    }

    for _ in 0..26_000_000 {
        let pos = (rng.gen_range(0..20000), rng.gen_range(0..11000));
        animals.push(Moose {
            pos,
            rest_area: ((rng.gen_range(pos.0-1000..pos.0+1000), rng.gen_range(pos.1-1000..pos.1+1000)), rng.gen_range(0..1000)),
        });
    }
}
