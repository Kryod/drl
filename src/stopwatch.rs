use std::time::{ Instant, Duration };

#[derive(Default)]
pub struct Stopwatch {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.end_time = None;
    }

    pub fn elapsed(&self) -> Option<Duration> {
        if let Some(start) = self.start_time {
            if let Some(end) = self.end_time {
                Some(end - start)
            } else {
                None
            }
        } else {
            None
        }
    }
}
