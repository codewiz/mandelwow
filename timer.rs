use std::time::{Duration, Instant};

#[cfg(feature = "editor")]
use rust_rocket;

#[cfg(feature = "editor")]
type Rocket = rust_rocket::client::RocketClient;

#[cfg(not(feature = "editor"))]
type Rocket = ();

#[cfg(feature = "editor")]
const BPS: f32 = 10.0;

#[derive(Debug)]
pub struct Timer {
    pub t: f32,          /// Simulation time (starts from 0 and does not advance while on pause).
    pub now: Instant,    /// Wall time, use instead of Instant::now() for frame-consistent time.
    prev_time: Instant,  /// Time of previous frame.
    frame: u32,          /// Frame count, starts from 0 and does not increment while on pause.

    last_report_time: Instant,
    last_report_frame: u32,

    accum_draw_time: Duration,
    accum_idle_time: Duration,

    pub pause: bool,

    pub rocket: Option<Rocket>,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Timer {
            t: 0.0,
            now,
            prev_time: now,
            frame: 0,
            last_report_time: now,
            last_report_frame: 0,
            accum_draw_time: Duration::default(),
            accum_idle_time: Duration::default(),
            pause: false,
            rocket: Timer::init_rocket(),
        }
    }

    // To be called once per frame, just before rendering
    pub fn update(&mut self) {
        self.prev_time = self.now;
        self.now = Instant::now();
        if !self.pause {
            // Increment simulation time
            let frame_time = self.now - self.prev_time;
            self.t += frame_time.as_secs_f32();
            self.frame += 1;
        }
        self.poll_rocket();
        self.maybe_report();
    }

    #[cfg(not(feature = "logging"))]
    fn maybe_report(&mut self) {}

    #[cfg(feature = "logging")]
    fn maybe_report(&mut self) {
        if self.now - self.last_report_time > Duration::from_secs(5) {
            self.report(self.now);
            self.last_report_time = self.now;
            self.last_report_frame = self.frame;
            self.accum_draw_time = Duration::new(0, 0);
            self.accum_idle_time = Duration::new(0, 0);
        }
    }

    #[cfg(feature = "logging")]
    fn report(&self, now: Instant) {
        fn millis(d : Duration) -> f32 {
            d.as_secs() as f32 * 1e3 + d.subsec_nanos() as f32 / 1e6
        }
        let frames_done = self.frame - self.last_report_frame;
        let fps = frames_done as f32 / (now - self.last_report_time).as_secs() as f32;
        let avg_draw_time = millis(self.accum_draw_time / frames_done);
        let avg_idle_time = millis(self.accum_idle_time / frames_done);
        println!("fps={:.1} draw={:.1}ms idle={:.1}ms", fps, avg_draw_time, avg_idle_time);
    }

    #[cfg(not(feature = "editor"))]
    fn init_rocket() -> Option<Rocket> { None }

    #[cfg(not(feature = "editor"))]
    fn poll_rocket(&mut self) {}

    #[cfg(feature = "editor")]
    fn init_rocket() -> Option<Rocket> {
        Rocket::new().ok()
    }

    #[cfg(feature = "editor")]
    fn poll_rocket(&mut self) {
        use rust_rocket::client::Event;

        match self.rocket {
            Some(ref mut rocket) => {
                let current_row = (self.t * BPS) as u32;
                if let Some(event) = rocket.poll_events().unwrap() {
                    match event {
                        Event::SetRow(row) => {
                            println!("SetRow (row: {:?})", row);
                            self.t = row as f32 / BPS;
                        }
                        Event::Pause(_) => {
                            let track1 = rocket.get_track_mut("test").unwrap();
                            println!("Pause (value: {:?}) (row: {:?})", track1.get_value(current_row as f32), current_row);
                        }
                        _ => (),
                    }
                    println!("{:?}", event);
                }
            }
            None => ()
        }
    }
}
