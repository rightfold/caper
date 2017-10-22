use cgmath::{Vector2, Vector3};
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub map: map::Map,
    pub entities: entity::Entities,
}

impl World {
    pub fn new(player_position: Vector2<f32>) -> Self {
        World{
            map: map::Map::new(),
            entities: entity::Entities::new(player_position),
        }
    }

    pub fn camera_position(&self) -> Vector3<f32> {
        self.entities.player.position.extend(0.0)
            + Vector3::new(0.0, -8.0, 4.0)
    }

    pub fn player_position(&self) -> Vector2<f32> {
        self.entities.player.position
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R) {
        self.entities.simulate(rng);
    }
}

pub mod entity;
pub mod graphics;
pub mod map;
