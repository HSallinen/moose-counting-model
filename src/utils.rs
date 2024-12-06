use std::ops::{Add, Sub, Mul};

pub fn distance_squared<T: Sub + Copy>(pos1: (T, T), pos2: (T, T))
    -> <<<T as Sub>::Output as Mul>::Output as Add>::Output
    where <T as Sub>::Output: Mul,
    <<T as Sub>::Output as Mul>::Output: Add
{
    ((pos1.0 - pos2.0) * (pos1.0 - pos2.0)) + ((pos1.1 - pos2.1) * (pos1.1 - pos2.1))
}
