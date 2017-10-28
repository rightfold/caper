use rand::Rng;

use world::World;

pub struct SimulationState {
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{}
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32, world: &mut World) {
    }
}
