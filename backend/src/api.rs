use crate::{alarm::AlarmT, config::AlarmConfigFile};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// TODO: Make API return json for errors

#[derive(Serialize, Deserialize)]
struct JsonError {
    _status: String,
    error: String,
}

impl JsonError {
    fn from(error: &str) -> Self {
        Self {
            error: error.to_string(),
            _status: "error".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct JsonSuccess {
    _status: String,
    message: String,
}

impl JsonSuccess {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
            _status: "success".to_string(),
        }
    }
}

#[post("/alarm/create")]
async fn api_alarm_create(req_body: String, data: web::Data<AppState>) -> impl Responder {
    // open a file, write the date and time to it, and close it
    // let now = chrono::Local::now();

    // if the date and time could not be parsed, return an error to the client
    let new_alarm = match AlarmT::from_json(&req_body) {
        Ok(new_alarm) if AlarmT::validate_json(&req_body) => new_alarm,
        Err(_) => {
            return HttpResponse::BadRequest().json(JsonError::from("Incorrect alarm format"))
        }
        _ => return HttpResponse::BadRequest().json(JsonError::from("Time given is impossible")),
    };

    let mut alarm_state = data.alarm_state.lock().unwrap();

    match alarm_state.has_alarm(new_alarm.clone()) {
        Some(_) => return HttpResponse::BadRequest().json(JsonError::from("Alarm already exists")),
        None => {
            alarm_state.add_alarm(new_alarm);
            return HttpResponse::Ok().json(JsonSuccess::from("Alarm created"));
        }
    };
}

#[post("/alarm/delete")]
async fn api_alarm_delete(req_body: String, data: web::Data<AppState>) -> impl Responder {
    // first check if the requested alarm is just a number
    // if it is, delete the alarm at that index
    match req_body.parse::<usize>() {
        Ok(index) => {
            let mut alarm_state = data.alarm_state.lock().unwrap();
            match alarm_state.del_alarm_by_index(index) {
                Some(_) => return HttpResponse::Ok().json(JsonSuccess::from("Alarm deleted")),
                None => return HttpResponse::BadRequest().json(JsonError::from("Alarm not found")),
            };
        }
        Err(_) => (),
    }

    // if the date and time could not be parsed, return an error to the client
    let del_alarm = match AlarmT::from_json(&req_body) {
        Ok(del_alarm) if AlarmT::validate_json(&req_body) => del_alarm,
        Err(_) => {
            return HttpResponse::BadRequest().json(JsonError::from("Incorrect alarm format"))
        }
        _ => return HttpResponse::BadRequest().json(JsonError::from("Time given is impossible")),
    };

    match data.alarm_state.lock().unwrap().del_alarm(del_alarm) {
        None => return HttpResponse::BadRequest().json(JsonError::from("Alarm not found")),
        Some(_) => return HttpResponse::Ok().json(JsonSuccess::from("Alarm deleted")),
    };
}

#[get("/alarms")]
async fn api_alarms(data: web::Data<AppState>) -> impl Responder {
    let alarm_state = data.alarm_state.lock().unwrap();
    HttpResponse::Ok().json(alarm_state.alarms.clone())
}

struct AppState {
    alarm_state: Mutex<AlarmConfigFile>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // read the alarm config file
    let alarm_config = AlarmConfigFile::from_file("alarm_config.json")
        // if the file could not be opened, make a new one
        .unwrap_or_else(|_| {
            let new_config = AlarmConfigFile::new("alarm_config.json");
            new_config.to_file("alarm_config.json").unwrap();
            new_config
        });

    cfg.app_data(web::Data::new(AppState {
        alarm_state: Mutex::new(alarm_config),
    }));
    cfg.service(
        web::scope("/api")
            .service(api_alarm_create)
            .service(api_alarm_delete)
            .service(api_alarms),
    );
}
