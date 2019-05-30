use std::fmt;

const MINS: i32 = 60;
const HOURS: i32 = 24;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

// My original implementation (canonical congruence):
// fn pmod(x: i32, n: i32) -> i32 {
//     (x % n + n) % n
// }
use num_integer::mod_floor;

// My original implementation:
// fn i32floor(x: i32, d: i32) -> i32 {
//     (x as f32 / d as f32).floor() as i32
// }
use num_integer::div_floor;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = mod_floor(hours, HOURS) + div_floor(minutes, MINS);
        Self {
            minutes: mod_floor(minutes, MINS),
            hours: mod_floor(hours, HOURS),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let hours = self.hours + div_floor(minutes, MINS);
        Self {
            minutes: mod_floor(minutes, MINS),
            hours: mod_floor(hours, HOURS),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
