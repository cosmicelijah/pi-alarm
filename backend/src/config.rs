use serde::{Deserialize, Serialize};

pub struct AlarmConfigFile {
    pub alarms: Vec<AlarmT>,
}
