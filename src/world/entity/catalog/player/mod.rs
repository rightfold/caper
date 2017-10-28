use cgmath::{Rad, Vector2};

#[derive(Debug)]
pub struct Player {
    pub position: Vector2<f32>,
    pub angle: Rad<f32>,
    pub sword_state: SwordState,
}

#[derive(Clone, Copy, Debug)]
pub enum SwordState {
    /// The player is carrying the sword.
    Carrying,

    /// The player is swinging the swor, with an animation time somewhere
    /// between 0.0 and 1.0.
    Swinging(f32),
}

impl Player {
    pub fn new(position: Vector2<f32>) -> Self {
        Player{
            position,
            angle: Rad(0.0),
            sword_state: SwordState::Carrying,
        }
    }
}

pub mod graphics;
pub mod simulation;
