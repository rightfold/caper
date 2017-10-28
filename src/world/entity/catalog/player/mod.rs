use cgmath::{Rad, Vector2};

#[derive(Debug)]
pub struct Player {
    pub position: Vector2<f32>,
    pub angle: Rad<f32>,
}

impl Player {
    pub fn new(position: Vector2<f32>) -> Self {
        Player{position, angle: Rad(0.0)}
    }
}

pub mod graphics;
pub mod simulation;
