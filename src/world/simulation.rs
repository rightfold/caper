use rand::Rng;

use input::Input;
use world::entity;
use world::World;

pub struct SimulationState {
    player: entity::catalog::player::simulation::SimulationState,

    spiders: entity::catalog::spider::simulation::SimulationState,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState{
            player: entity::catalog::player::simulation::SimulationState::new(),

            spiders: entity::catalog::spider::simulation::SimulationState::new(),
        }
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32, input: &Input, world: &mut World) {
        self.player.simulate(dt, input, world);
        self.spiders.simulate(rng, dt, &mut world.spiders);
    }
}
