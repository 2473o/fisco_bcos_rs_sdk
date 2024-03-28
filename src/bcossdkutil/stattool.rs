// use time::Tm;
use chrono::DateTime;

pub struct StatTime {
    pub time_begin: DateTime<chrono::Utc>,
    pub time_end: DateTime<chrono::Utc>,
}

impl StatTime {
    pub fn begin() -> Self {
        StatTime {
            // time_begin : time::now(),
            // time_end : time::now()
            time_begin: chrono::Utc::now(),
            time_end: chrono::Utc::now(),
        }
    }
    pub fn done(&mut self) {
        // self.time_end = time::now();
        self.time_end = chrono::Utc::now();
    }
    pub fn used_ms(&self) -> i64 {
        let time_used = self.time_end - self.time_begin;
        return time_used.num_milliseconds();
    }
}
