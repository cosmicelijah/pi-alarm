use serde::{ser::Error, Deserialize, Serialize};

use crate::alarm::AlarmT;

#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmConfigFile {
    pub alarms: Vec<AlarmT>,

    // #[serde(skip_serializing)] // for whatever reason, when skipping this, it skips the entire struct
    pub filename: String,
}

impl AlarmConfigFile {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            alarms: Vec::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_file(file: &str) -> Result<Self, serde_json::Error> {
        let file = match std::fs::File::open(file) {
            Ok(file) => file,
            Err(_) => return Err(serde_json::Error::custom("Could not open file")),
        };

        let reader = std::io::BufReader::new(file);

        serde_json::from_reader(reader)
    }

    pub fn to_file(&self, file: &str) -> Result<(), serde_json::Error> {
        let file = match std::fs::File::create(file) {
            Ok(file) => file,
            Err(_) => return Err(serde_json::Error::custom("Could not open file")),
        };

        let writer = std::io::BufWriter::new(file);

        serde_json::to_writer(writer, self)
    }

    pub fn add_alarm(&mut self, alarm: AlarmT) {
        self.alarms.push(alarm);
        let _ = self.to_file(&self.filename); // specifically ignoring the possibility of an error
    }

    pub fn has_alarm(&self, alarm: AlarmT) -> Option<usize> {
        self.alarms.iter().position(|x| *x == alarm)
    }

    pub fn del_alarm(&mut self, alarm: AlarmT) -> Option<AlarmT> {
        let index = self.has_alarm(alarm);
        match index {
            Some(index) => self.del_alarm_by_index(index),
            None => None,
        }
    }

    pub fn del_alarm_by_index(&mut self, index: usize) -> Option<AlarmT> {
        if index < self.alarms.len() {
            let alarm = self.alarms.remove(index);
            let _ = self.to_file(&self.filename); // specifically ignoring the possibility of an error
            Some(alarm)
        } else {
            None
        }
    }
}
