use rand::Rng;

use world::entity;
use world::World;

pub struct SimulationState {
    spiders: entity::catalog::spider::simulation::SimulationState,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{
            spiders: entity::catalog::spider::simulation::SimulationState::new(),
        }
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32, world: &mut World) {
        self.spiders.simulate(rng, dt, &mut world.spiders);
    }
}
