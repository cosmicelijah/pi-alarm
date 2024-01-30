use std::{ops::Add, sync::Mutex, thread, time::Duration};

use serde::{Deserialize, Serialize};

use chrono::{Duration, Local, NaiveTime};

use rppal::gpio::{self, Gpio};

// Pin 16 on the board
const GPIO_ALARM_TRIGGER: u8 = 23;
static ALARM_PIN: Mutex<gpio::pin::Pin> =
    Mutex::new(Gpio::new()?.get(GPIO_ALARM_TRIGGER)?.into_output());

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

    fn activate_alarm(&self, arc_pin: Arc<Mutex<gpio::pin::Pin>>) {
        // Try lock, if success, ring alarm, else return since alarm is already ringing
        if let mut alarm_pin = arc_pin.try_lock().unwrap() {
            println!("Alarm ringing!!!");
            thread::sleep(Duration::from_secs(90));
        } else {
            return;
        }
    }

    fn set_alarm_thread(&self, alarm_time: NaiveTime) {
        while self.enabled {
            let now = Local::now().time();

            // Alarm should be set into the future
            let mut duration = alarm_time.signed_duration_since(now);

            if duration < Duration::zero() {
                duration = duration.add(Duration::days(1));
            }

            println!("Alarm set for {} seconds into the future", duration);

            let dur = match duration.to_std() {
                Ok(dur) => dur,
                Err(_) => {
                    eprintln!("Something is fucked");
                    return;
                }
            };

            // Turn off alarm if it was disabled during sleep
            if !self.enabled {
                println!("Alarm was disabled during sleep");
                return;
            }
            println!("Waiting for the right time...");
            thread::sleep(dur);

            // Lock gpio pin usage and ring alarm so that two alarms can't ring at once
            /**
             *  If the pin is locked and the alarm wants to ring, simply skip and go back to sleep
             *  Reasoning:
             *      If unlocked -> no alarm ringing
             *      If locked -> alarm already ringing, no need to ring,
             *          will just annoy user for having to turn off two concurrent alarms,
             *          might also just break the program if two functions are trying to modify a pin,
             *          better to just lock the pin just in case
             */
            let arc_pin: Arc<Mutex<gpio::pin::Pin>> = Arc::new(ALARM_PIN);
            self.activate_alarm(arc_pin);
        }
    }

    pub fn set_alarm(&self) -> bool {
        let alarm_time = match NaiveTime::parse_from_str(&self.to_string(), "%H:%M") {
            Ok(alarm_time) => alarm_time,
            Err(_) => {
                eprintln!("Could not parse alarm: {}", self.to_string());
                return false;
            }
        };

        let self_clone = self.clone();

        println!("Creating thread for alarm");

        thread::spawn(move || {
            self_clone.set_alarm_thread(alarm_time);
        });

        return true;
    }
}
