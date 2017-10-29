#[derive(Clone, Debug)]
pub struct Since {
    since_last: f32,
}

impl Since {
    pub fn new() -> Self {
        Since{since_last: 0.0}
    }

    pub fn catch_up<F: FnMut()>(&mut self, interval: f32, dt: f32, mut body: F) {
        self.since_last += dt;
        while self.since_last > interval {
            self.since_last -= interval;
            body();
        }
    }
}
