use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DaysT {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlarmT {
    pub hour: u32,
    pub minute: u32,
    pub days: Vec<DaysT>,
}
