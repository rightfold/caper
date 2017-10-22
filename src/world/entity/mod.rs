use cgmath::Vector2;
use rand::Rng;

#[derive(Debug)]
pub struct Entities {
    pub player: player::Player,
    pub spiders: spider::SpiderSet,
}

impl Entities {
    pub fn new(player_position: Vector2<f32>) -> Self {
        Entities{
            player: player::Player::new(player_position),
            spiders: spider::SpiderSet::new(),
        }
    }

    pub fn simulate<R: Rng>(&mut self, rng: &mut R) {
        self.spiders.simulate(rng);
    }
}

pub mod collections;
pub mod graphics;
pub mod player;
pub mod spider;
