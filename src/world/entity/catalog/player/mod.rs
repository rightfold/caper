use cgmath::Vector2;

#[derive(Debug)]
pub struct Player {
    pub position: Vector2<f32>,
}

impl Player {
    pub fn new(position: Vector2<f32>) -> Self {
        Player{position}
    }
}

pub mod graphics;
