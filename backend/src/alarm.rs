use std::{ops::Add, str::FromStr, thread};

use serde::{Deserialize, Serialize};

use chrono::{Duration, Local, NaiveTime};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum DaysT {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AlarmT {
    pub hour: u8,
    pub minute: u8,
    pub days: Vec<DaysT>,
    pub enabled: bool,
}

// add a trait to alarm to get the json representation of the alarm
impl AlarmT {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<AlarmT, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.hour, self.minute)
    }

    pub fn validate_json(json: &str) -> bool {
        match serde_json::from_str::<AlarmT>(json) {
            Ok(alarm) if alarm.hour < 24 && alarm.minute < 60 => true,
            _ => false,
        }
    }

    async fn set_alarm_thread(&self, duration: Duration) {
        while self.enabled {
            let dur = match duration.to_std() {
                Ok(dur) => dur,
                Err(_) => {
                    eprintln!("Something is fucked");
                    return;
                }
            };

            // Turn off alarm if it was disabled during sleep
            if !self.enabled {
                return;
            }
            thread::sleep(dur);
        }
    }

    pub fn set_alarm(&mut self, alarm: AlarmT) -> bool {
        let alarm_time = match NaiveTime::parse_from_str(&alarm.to_string(), "%H:%M") {
            Ok(alarm_time) => alarm_time,
            Err(_) => {
                eprintln!("Could not parse alarm: {}", alarm.to_string());
                return false;
            }
        };

        let now = Local::now().time();

        // Alarm should be set into the future
        let mut duration = alarm_time.signed_duration_since(now);

        if duration < Duration::zero() {
            duration = duration.add(Duration::days(1));
        }

        println!("Alarm set for {} seconds into the future", duration);

        thread::spawn(move || {
            alarm.set_alarm_thread(duration);
        });

        return true;
    }
}
