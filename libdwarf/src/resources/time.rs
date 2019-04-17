/// Basically a copy of amethyst_core::timing::time, but separated so we can
/// use the same resource outside of a amethyst rendering environment.
use std::time::{ Duration, Instant };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    /// Time elapsed since the last frame in seconds.
    delta_seconds: f32,
    /// Time elapsed since the last frame.
    delta_time: Duration,
    /// The total number of frames that have been played in this session.
    frame_number: u64,
    /// Time elapsed since simulation has started, taking the speed multiplier into
    /// account.
    absolute_time: Duration,
    /// Time multiplier. Affects returned delta_seconds, delta_time, and absolute_time.
    time_scale: f32,
}

impl Time {
    /// Gets the time difference between frames in seconds.
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// Gets the time difference between frames.
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Gets the time since the start of the game, taking into account the speed multiplier.
    pub fn absolute_time(&self) -> Duration {
        self.absolute_time
    }

    /// Gets the current time speed multiplier.
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Gets the total number of frames that have been played in this session.
    /// Sets both `delta_seconds` and `delta_time` based on the seconds given.
    ///
    /// This should only be called by the engine.  Bad things might happen if you call this in
    /// your game.
    pub fn set_delta_seconds(&mut self, secs: f32) {
        self.delta_seconds = secs * self.time_scale;
        self.delta_time = secs_to_duration(secs * self.time_scale);

        self.absolute_time += self.delta_time;
    }

    /// Sets both `delta_time` and `delta_seconds` based on the duration given.
    ///
    /// This should only be called by the engine.  Bad things might happen if you call this in
    /// your game.
    pub fn set_delta_time(&mut self, time: Duration) {
        self.delta_seconds = duration_to_secs(time) * self.time_scale;
        self.delta_time = secs_to_duration(duration_to_secs(time) * self.time_scale);
        self.absolute_time += self.delta_time;
    }

    /// Sets the time multiplier that affects how time values are computed,
    /// effectively slowing or speeding up the sim.
    ///
    /// ## PANICS
    /// This will panic if multiplier is NaN, Infinity,
    /// or less than or equal to 0.
    pub fn set_time_scale(&mut self, multiplier: f32) {
        use std::f32::INFINITY;
        assert!(multiplier >= 0.0);
        assert!(multiplier != INFINITY);
        self.time_scale = multiplier;
    }

    /// Increments the current frame number by 1
    ///
    /// Should only get called by the sim when on every frame.
    pub fn increment_frame_number(&mut self) {
        self.frame_number += 1;
    }
}

impl Default for Time {
    fn default() -> Time {
        Time {
            delta_seconds: 0.0,
            delta_time: Duration::from_secs(0),
            frame_number: 0,
            time_scale: 1.0,
            absolute_time: Duration::default(),
        }
    }
}

/// A stopwatch which accurately measures elapsed time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stopwatch {
    /// Initial state with an elapsed time value of 0 seconds.
    Waiting,
    /// Stopwatch has started counting the elapsed time since this `Instant`
    /// and accumuluated time from previous start/stop cycles `Duration`.
    Started(Duration, Instant),
    /// Stopwatch has been stopped and reports the elapsed time `Duration`.
    Ended(Duration),
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch::Waiting
    }
}

impl Stopwatch {
    /// Creates a new stopwatch.
    pub fn new() -> Stopwatch {
        Default::default()
    }

    /// Retrieves the elapsed time.
    pub fn elapsed(&self) -> Duration {
        match *self {
            Stopwatch::Waiting => Duration::new(0, 0),
            Stopwatch::Started(dur, start) => dur + start.elapsed(),
            Stopwatch::Ended(dur) => dur,
        }
    }

    /// Stops, resets, and starts the stopwatch again.
    pub fn restart(&mut self) {
        *self = Stopwatch::Started(Duration::new(0, 0), Instant::now());
    }

    /// Starts, or resumes, measuring elapsed time. If the stopwatch has been
    /// started and stopped before, the new results are compounded onto the
    /// existing elapsed time value.
    ///
    /// Note: Starting an already running stopwatch will do nothing.
    pub fn start(&mut self) {
        match *self {
            Stopwatch::Waiting => self.restart(),
            Stopwatch::Ended(dur) => {
                *self = Stopwatch::Started(dur, Instant::now());
            }
            _ => {}
        }
    }

    /// Stops measuring elapsed time.
    ///
    /// Note: Stopping a stopwatch that isn't running will do nothing.
    pub fn stop(&mut self) {
        if let Stopwatch::Started(dur, start) = *self {
            *self = Stopwatch::Ended(dur + start.elapsed());
        }
    }

    /// Clears the current elapsed time value.
    pub fn reset(&mut self) {
        *self = Stopwatch::Waiting;
    }
}

/// Converts a Duration to the time in seconds.
pub fn duration_to_secs(duration: Duration) -> f32 {
    duration.as_secs() as f32 + (duration.subsec_nanos() as f32 / 1.0e9)
}

/// Converts a Duration to the time in seconds in an f64.
pub fn duration_to_secs_f64(duration: Duration) -> f64 {
    duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1.0e9)
}

/// Converts a time in seconds to a duration
pub fn secs_to_duration(secs: f32) -> Duration {
    Duration::new(secs as u64, ((secs % 1.0) * 1.0e9) as u32)
}

/// Converts a Duration to nanoseconds
pub fn duration_to_nanos(duration: Duration) -> u64 {
    (duration.as_secs() * 1_000_000_000) + u64::from(duration.subsec_nanos())
}

/// Converts nanoseconds to a Duration
pub fn nanos_to_duration(nanos: u64) -> Duration {
    Duration::new(nanos / 1_000_000_000, (nanos % 1_000_000_000) as u32)
}