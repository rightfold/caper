use std::ops::Add;

use rand::Rng;
use rand::distributions::range::SampleRange;

pub fn gen_range_base<R, T>(rng: &mut R, base: T, range: (T, T)) -> T
    where R: Rng, T: Add<Output=T> + PartialOrd + SampleRange {
    base + rng.gen_range(range.0, range.1)
}
