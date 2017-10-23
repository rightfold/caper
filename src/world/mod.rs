use cgmath::{Vector2, Vector3};
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub map: map::Map,

    pub player: entity::catalog::player::Player,

    pub spiders: entity::catalog::spider::SpiderSet,
}

impl World {
    pub fn new(player_position: Vector2<f32>) -> Self {
        World{
            map: map::Map::new(),

            player: entity::catalog::player::Player::new(player_position),

            spiders: entity::catalog::spider::SpiderSet::new(),
        }
    }

    pub fn camera_position(&self) -> Vector3<f32> {
        self.player.position.extend(0.0) + Vector3::new(0.0, -8.0, 4.0)
    }

    pub fn player_position(&self) -> Vector2<f32> {
        self.player.position
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R) {
        self.spiders.simulate(rng);
    }
}

pub mod entity;
pub mod graphics;
pub mod map;
