use rand::Rng;

use world::World;

pub struct SimulationState {
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{}
    }

    pub fn simulate<R: Rng>(&mut self, _rng: &mut R, _dt: f32, _world: &mut World) {
    }
}
