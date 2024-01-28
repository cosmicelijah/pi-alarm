mod alarm;
mod api;
mod config;

use actix_web::{App, HttpServer};


 // Add the Write trait to the list of imported traits

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .configure(api::config)
            .service(actix_files::Files::new("/", "../frontend/build").index_file("index.html"))
    })
    .bind(("0.0.0.0", 6969))? // DONE: needs better port?
    .run()
    .await
}
