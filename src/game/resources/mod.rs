
#[derive(Default)]
pub struct GameTick {
    pub tick_delta: f32,
    pub last_tick: f32,
}

impl GameTick {
    pub fn new(tick_delta: f32) -> Self {
        GameTick {
            tick_delta,
            last_tick: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.last_tick = self.tick_delta;
    }
}