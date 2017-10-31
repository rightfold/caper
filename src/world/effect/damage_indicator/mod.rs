use cgmath::{Matrix4, Vector2};

soa! {
    DamageIndicatorSet,

    DamageIndicatorId,

    array positions: Vector2<f32>,
    array ages: f32,

    scalar graphics: graphics::Graphics = graphics::Graphics::new(),
}

impl DamageIndicatorSet {
    pub fn spawn(&mut self, position: Vector2<f32>, _damage: f32) -> DamageIndicatorId {
        soa_spawn! {
            self,
            positions: position,
            ages: 0.0,
        }
    }

    pub fn draw(&self, pmat: Matrix4<f32>, vmat: Matrix4<f32>,
                mmat: Matrix4<f32>) {
        self.scalars.graphics.draw(self, pmat, vmat, mmat);
    }
}

mod graphics;
