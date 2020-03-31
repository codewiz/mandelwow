use std::time::{Duration, Instant};

#[cfg(feature = "editor")]
use rust_rocket;

#[cfg(feature = "editor")]
type Rocket = rust_rocket::Rocket;

#[cfg(not(feature = "editor"))]
type Rocket = ();

#[cfg(feature = "editor")]
const BPS: f32 = 10.0;

#[derive(Debug)]
pub struct Timer {
    pub t: f32,

    frame: u32,

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
            frame: 0,
            last_report_time: now,
            last_report_frame: 0,
            accum_draw_time: Duration::default(),
            accum_idle_time: Duration::default(),
            pause: false,
            rocket: Timer::init_rocket(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.poll_rocket();
        if !self.pause {
            // Increment time
            self.t += 0.01;
            self.frame += 1;
        }
        self.maybe_report(now);
    }

    #[cfg(not(feature = "logging"))]
    fn maybe_report(&mut self, _: Instant) {}

    #[cfg(feature = "logging")]
    fn maybe_report(&mut self, now: Instant) {
        if now - self.last_report_time > Duration::from_secs(5) {
            self.report(now);
            self.last_report_time = now;
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
        match self.rocket {
            Some(ref mut rocket) => {
                let current_row = (self.t * BPS) as u32;
                if let Some(event) = rocket.poll_events() {
                    match event {
                        rust_rocket::Event::SetRow(row) => {
                            println!("SetRow (row: {:?})", row);
                            self.t = row as f32 / BPS;
                        }
                        rust_rocket::Event::Pause(_) => {
                            let track1 = rocket.get_track_mut("test");
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
