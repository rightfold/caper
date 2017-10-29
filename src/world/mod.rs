use cgmath::{EuclideanSpace, Matrix4, One, Point3, Vector2, Vector3};

use world::monster::MonsterSet;

#[derive(Debug)]
pub struct World {
    pub map: map::Map,

    pub player: player::Player,

    pub spiders: monster::spider::SpiderSet,
    pub gas_spores: monster::gas_spore::GasSporeSet,
}

impl World {
    pub fn new() -> Self {
        World{
            map: map::Map::new(),

            player: player::Player::new(Vector2::new(0.0, 0.0)),

            spiders: monster::spider::SpiderSet::new(),
            gas_spores: monster::gas_spore::GasSporeSet::new(),
        }
    }

    pub fn draw(&self, pmat: Matrix4<f32>) {
        let camera_position =
            self.player.position.extend(0.0)
            + Vector3::new(0.0, -7.0, 9.0);

        let vmat = Matrix4::look_at(
            Point3::from_vec(camera_position),
            Point3::from_vec(self.player.position.extend(0.0)),
            Vector3::new(0.0, 0.0, 1.0),
        );

        let mmat = Matrix4::one();

        let light_position = self.player.position;

        self.map.draw(pmat, vmat, mmat, light_position);

        self.player.draw(pmat, vmat, mmat, light_position);

        self.spiders.draw(pmat, vmat, mmat, light_position);
        self.gas_spores.draw(pmat, vmat, mmat, light_position);
    }
}

pub mod item;
pub mod map;
pub mod monster;
pub mod player;
