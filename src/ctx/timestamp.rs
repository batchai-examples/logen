use chrono::{DateTime, Utc, Duration};
use rand::Rng;

use crate::def::TimestampDef;


pub struct Timestamp<'a> {
    def: &'a TimestampDef,
    value: DateTime<Utc>,
    interval: i64,
}

impl<'a> Timestamp<'a> {
    pub fn new(def: &TimestampDef, lines: u64) -> Timestamp {
        Timestamp {
            def,
            value: def.begin,
            interval: (def.end.timestamp_millis() - def.begin.timestamp_millis()) / lines as i64,
        }
    }

    pub fn next(&mut self) -> String {
        let mut i = 0;
        let max = self.interval;
        while i < 10 {
            let itvl = rand::thread_rng().gen_range(0..((max as f64) * 1.8) as i64);
            let new_value = self.value + Duration::milliseconds(itvl);
            if new_value < self.def.end {
                self.value = new_value;
                return self.value.format(self.def.format.as_str()).to_string();
            }

            i = i+1;
        }

        self.value = self.def.end;
        return self.value.format(self.def.format.as_str()).to_string();
    }
}
