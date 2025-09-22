use std::time::Duration;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Tick {
    tick: usize,
}

impl Tick {
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

impl Add for Tick {
    type Output = Tick;
    fn add(self, other: Tick) -> Tick {
        Tick::new(self.tick + other.tick)
    }
}

impl AddAssign for Tick {
    fn add_assign(&mut self, other: Tick) {
        self.tick += other.tick;
    }
}

impl Sub for Tick {
    type Output = Tick;
    fn sub(self, other: Tick) -> Tick {
        Tick::new(self.tick.saturating_sub(other.tick))
    }
}

impl SubAssign for Tick {
    fn sub_assign(&mut self, other: Tick) {
        self.tick = self.tick.saturating_sub(other.tick);
    }
}

impl Mul<usize> for Tick {
    type Output = Tick;
    fn mul(self, rhs: usize) -> Tick {
        Tick::new(self.tick * rhs)
    }
}

impl MulAssign<usize> for Tick {
    fn mul_assign(&mut self, rhs: usize) {
        self.tick *= rhs;
    }
}

impl Div<usize> for Tick {
    type Output = Tick;
    fn div(self, rhs: usize) -> Tick {
        Tick::new(self.tick / rhs)
    }
}

impl DivAssign<usize> for Tick {
    fn div_assign(&mut self, rhs: usize) {
        self.tick /= rhs;
    }
}