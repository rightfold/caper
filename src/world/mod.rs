use cgmath::{EuclideanSpace, Matrix4, One, Point3, Vector2, Vector3};

use world::monster::MonsterSet;

#[derive(Debug)]
pub struct World {
    pub map: map::Map,

    pub damage_indicators: effect::damage_indicator::DamageIndicatorSet,

    pub player: player::Player,

    pub spiders: monster::spider::SpiderSet,
    pub gas_spores: monster::gas_spore::GasSporeSet,

    pub camera_target: Vector2<f32>,
}

impl World {
    pub fn new() -> Self {
        World{
            map: map::Map::new(),

            damage_indicators: effect::damage_indicator::DamageIndicatorSet::new(),

            player: player::Player::new(Vector2::new(0.0, 0.0)),

            spiders: monster::spider::SpiderSet::new(),
            gas_spores: monster::gas_spore::GasSporeSet::new(),

            camera_target: Vector2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, pmat: Matrix4<f32>) {
        let vmat = Matrix4::look_at(
            Point3::from_vec(self.camera_target.extend(0.0) + Vector3::new(0.0, -6.0, 8.0)),
            Point3::from_vec(self.camera_target.extend(0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );

        let mmat = Matrix4::one();

        let light_position = self.player.position;

        self.map.draw(pmat, vmat, mmat, light_position);

        self.damage_indicators.draw(pmat, vmat, mmat);

        self.player.draw(pmat, vmat, mmat, light_position);

        self.spiders.draw(pmat, vmat, mmat, light_position);
        self.gas_spores.draw(pmat, vmat, mmat, light_position);
    }
}

pub mod effect;
pub mod item;
pub mod map;
pub mod monster;
pub mod player;
