use cgmath::{Vector2, Vector3};
use rand::Rng;

use input::Input;

macro_rules! world {
    ($($singular:ident : $plural:ident : $type:ident,)*) => {
        #[derive(Debug)]
        pub struct World {
            pub map: map::Map,

            pub player: player::Player,

            $(pub $plural: monster::catalog::$singular::$type,)*
        }

        impl World {
            pub fn new(player_position: Vector2<f32>) -> Self {
                World{
                    map: map::Map::new(),

                    player: player::Player::new(player_position),

                    $($plural: monster::catalog::$singular::$type::new(),)*
                }
            }

            pub fn camera_position(&self) -> Vector3<f32> {
                self.player.position.extend(0.0) + Vector3::new(0.0, -7.0, 9.0)
            }

            pub fn player_position(&self) -> Vector2<f32> {
                self.player.position
            }
        }

        pub mod graphics {
            use super::*;

            use cgmath::{EuclideanSpace, Matrix4, One, Point3, Vector3};

            pub struct GraphicsState {
                map: map::graphics::GraphicsState,

                player: player::graphics::GraphicsState,

                $($plural: monster::catalog::$singular::graphics::GraphicsState,)*
            }

            impl GraphicsState {
                pub fn new() -> Self {
                    GraphicsState{
                        map: map::graphics::GraphicsState::new(),

                        player: player::graphics::GraphicsState::new(),

                        $($plural: monster::catalog::$singular::graphics::GraphicsState::new(),)*
                    }
                }

                pub fn draw(&self, pmat: Matrix4<f32>, world: &World) {
                    let vmat = Matrix4::look_at(
                        Point3::from_vec(world.camera_position()),
                        Point3::from_vec(world.player_position().extend(0.0)),
                        Vector3::new(0.0, 0.0, 1.0),
                    );
                    let mmat = Matrix4::one();

                    let light_position = world.player_position();

                    self.map.draw(pmat, vmat, mmat, light_position, &world.map);

                    self.player.draw(pmat, vmat, mmat, light_position, &world.player);

                    $(self.$plural.draw(pmat, vmat, mmat, light_position, &world.$plural);)*
                }
            }
        }

        pub mod simulation {
            use super::*;

            pub struct SimulationState {
                player: player::simulation::SimulationState,

                $($plural: monster::catalog::$singular::simulation::SimulationState,)*
            }

            impl SimulationState {
                pub fn new() -> Self {
                    SimulationState{
                        player: player::simulation::SimulationState::new(),

                        $($plural: monster::catalog::$singular::simulation::SimulationState::new(),)*
                    }
                }

                pub fn simulate<R: Rng>(&mut self, rng: &mut R, dt: f32, input: &Input, world: &mut World) {
                    self.player.simulate(dt, input, world);
                    $(self.$plural.simulate(rng, dt, world);)*
                    $(monster::despawn_dead(&mut world.$plural);)*
                }
            }
        }
    };
}

with_each_monster_set!(world);

pub mod item;
pub mod map;
pub mod monster;
pub mod player;
