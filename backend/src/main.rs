mod alarm;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Write; // Add the Write trait to the list of imported traits

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/alarm")]
async fn api_alarm_post(_req_body: String) -> impl Responder {
    // open a file, write the date and time to it, and close it
    let now = chrono::Local::now();
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(false)
        .open("alarm_log.txt");

    // if the file could not be opened, return an error to the client
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Could not open file"),
    };

    let _ = writeln!(file, "{}", now.format("%Y-%m-%d %H:%M:%S"));

    return HttpResponse::Ok().append_header(("X-Ur-Mum-Is", "Fat")).body("Sex Alarm! *bangs gong repeatedly*");
}

#[get("/api/json-test")]
async fn api_json_test() -> impl Responder {
    let alarm = alarm::AlarmT {
        hour: 12,
        minute: 30,
        days: vec![alarm::DaysT::Monday, alarm::DaysT::Wednesday, alarm::DaysT::Friday],
    };

    let serialized = serde_json::to_string(&alarm).unwrap();

    HttpResponse::Ok().body(serialized)
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(api_alarm_post)
            .service(api_json_test)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 6969))? // DONE: needs better port?
    .run()
    .await
}

