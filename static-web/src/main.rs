use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind("127.0.0.1:9080")?
    .run()
    .await
}