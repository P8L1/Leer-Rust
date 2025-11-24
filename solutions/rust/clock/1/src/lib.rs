use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    min: i32,
}

impl Clock {
    fn normalize(total_min: i32) -> i32 {
        const DAY_MINUTES: i32 = 24 * 60;
        let mut m = total_min % DAY_MINUTES;
        if m < 0 {
            m += DAY_MINUTES;
        }
        m
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_min = hours * 60 + minutes;
        let min = Self::normalize(total_min);
        Clock { min }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_min = self.min + minutes;
        let min = Self::normalize(total_min);
        Clock { min }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.min / 60;
        let minutes = self.min % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
