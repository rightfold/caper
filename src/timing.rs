#[derive(Clone, Debug)]
pub struct Since(pub f32);

impl Since {
    pub fn new() -> Self {
        Since(0.0)
    }

    pub fn catch_up<F: FnMut()>(&mut self, interval: f32, dt: f32, mut body: F) {
        self.0 += dt;
        while self.0 > interval {
            self.0 -= interval;
            body();
        }
    }
}
