use cgmath::{Matrix4, Rad, Vector2};

#[derive(Debug)]
pub struct Player {
    pub graphics: graphics::Graphics,
    pub state: state::State,

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
            graphics: graphics::Graphics::new(),
            state: state::State::new(),

            position,
            angle: Rad(0.0),
            attack_state: None,
        }
    }

    pub fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mmat: Matrix4<f32>, light_position: Vector2<f32>) {
        self.graphics.draw(self, pmat, vmat, mmat, light_position);
    }
}

mod graphics;

mod state {
    #[derive(Debug)]
    pub struct State {
        pub since_attack: f32,
        pub attack_released: bool,
    }

    impl State {
        pub fn new() -> Self {
            State{
                since_attack: 0.0,
                attack_released: true,
            }
        }
    }
}
