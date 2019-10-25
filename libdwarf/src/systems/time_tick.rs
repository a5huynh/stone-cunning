use core::amethyst::ecs::{System, WriteExpect};

use crate::resources::time;

pub struct TimeTickSystem;
impl<'a> System<'a> for TimeTickSystem {
    type SystemData = (
        WriteExpect<'a, time::Time>,
        WriteExpect<'a, time::Stopwatch>,
    );

    fn run(&mut self, (mut time, mut stopwatch): Self::SystemData) {
        let elapsed = stopwatch.elapsed();
        time.increment_frame_number();
        time.set_delta_time(elapsed);

        stopwatch.stop();
        stopwatch.restart();
    }
}
