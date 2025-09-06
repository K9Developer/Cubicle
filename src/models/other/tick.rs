use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Tick {
    tick: usize,
}

pub trait TickTrait {
    fn get_tick(&self) -> usize;
    fn get_millis(&self) -> usize;
    fn get_seconds(&self) -> f64;
    fn get_minutes(&self) -> f64;
    fn get_hours(&self) -> f64;
    fn get_duration(&self) -> Duration;

    fn set_tick(&mut self, tick: usize);
    fn set_millis(&mut self, millis: usize);
    fn set_seconds(&mut self, seconds: f64);
    fn set_minutes(&mut self, minutes: f64);
    fn set_hours(&mut self, hours: f64);
    fn set_duration(&mut self, duration: Duration);

    fn add_ticks(&mut self, ticks: usize);
    fn add_millis(&mut self, millis: usize);
    fn add_seconds(&mut self, seconds: f64);
    fn add_minutes(&mut self, minutes: f64);
    fn add_hours(&mut self, hours: f64);
    fn add_duration(&mut self, duration: Duration);
}

impl TickTrait for Tick {
    fn get_tick(&self) -> usize {
        self.tick
    }

    fn get_millis(&self) -> usize {
        self.tick * 50
    }

    fn get_seconds(&self) -> f64 {
        self.tick as f64 / 20.0
    }

    fn get_minutes(&self) -> f64 {
        self.tick as f64 / (20.0 * 60.0)
    }

    fn get_hours(&self) -> f64 {
        self.tick as f64 / (20.0 * 60.0 * 60.0)
    }

    fn get_duration(&self) -> Duration {
        Duration::from_millis(self.get_millis() as u64)
    }

    fn set_tick(&mut self, tick: usize) {
        self.tick = tick;
    }

    fn set_millis(&mut self, millis: usize) {
        self.tick = millis / 50;
    }

    fn set_seconds(&mut self, seconds: f64) {
        self.tick = (seconds * 20.0) as usize;
    }

    fn set_minutes(&mut self, minutes: f64) {
        self.tick = (minutes * 60.0 * 20.0) as usize;
    }

    fn set_hours(&mut self, hours: f64) {
        self.tick = (hours * 60.0 * 60.0 * 20.0) as usize;
    }

    fn set_duration(&mut self, duration: Duration) {
        self.tick = (duration.as_millis() / 50) as usize;
    }

    fn add_ticks(&mut self, ticks: usize) {
        self.tick += ticks;
    }

    fn add_millis(&mut self, millis: usize) {
        self.tick += millis / 50;
    }

    fn add_seconds(&mut self, seconds: f64) {
        self.tick += (seconds * 20.0) as usize;
    }

    fn add_minutes(&mut self, minutes: f64) {
        self.tick += (minutes * 60.0 * 20.0) as usize;
    }

    fn add_hours(&mut self, hours: f64) {
        self.tick += (hours * 60.0 * 60.0 * 20.0) as usize;
    }

    fn add_duration(&mut self, duration: Duration) {
        self.tick += (duration.as_millis() / 50) as usize;
    }
}

impl Tick {
    pub fn new(ticks: usize) -> Self {
        Tick { tick: ticks }
    }
}