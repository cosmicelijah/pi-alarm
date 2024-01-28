use serde::{Deserialize, Serialize};

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
}


// add a trait to alarm to get the json representation of the alarm
impl AlarmT {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    
    pub fn from_json(json: &str) -> Result<AlarmT, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn validate_json(json: &str) -> bool {
        match serde_json::from_str::<AlarmT>(json) {
            Ok(alarm) if alarm.hour < 24 && alarm.minute < 60 => true,
            _ => false,
        }
    }
}
