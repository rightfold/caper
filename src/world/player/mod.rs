use cgmath::{Matrix4, Rad, Vector2};

#[derive(Debug)]
pub struct Player {
    pub graphics: graphics::Graphics,

    pub position: Vector2<f32>,
    pub angle: Rad<f32>,

    pub attack_state: AttackState,
    pub holding_attack: bool,
}

impl Player {
    pub fn new(position: Vector2<f32>) -> Self {
        Player{
            graphics: graphics::Graphics::new(),

            position,
            angle: Rad(0.0),

            attack_state: AttackState::NotAttacking,
            holding_attack: false,
        }
    }

    pub fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mmat: Matrix4<f32>, light_position: Vector2<f32>) {
        self.graphics.draw(self, pmat, vmat, mmat, light_position);
    }
}

#[derive(Clone, Debug)]
pub enum AttackState {
    Attacking(f32),
    NotAttacking,
}

mod graphics;
