use cgmath::{Rad, Vector2};

#[derive(Debug)]
pub struct Player {
    pub position: Vector2<f32>,
    pub angle: Rad<f32>,
    pub attack_state: Option<AttackState>,
}

#[derive(Clone, Copy, Debug)]
pub struct AttackState {
    pub progress: f32,
}

impl Player {
    pub fn new(position: Vector2<f32>) -> Self {
        Player{
            position,
            angle: Rad(0.0),
            attack_state: None,
        }
    }
}

pub mod graphics;
pub mod simulation;
