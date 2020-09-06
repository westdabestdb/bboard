mod server;

use crate::server::app_config;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  HttpServer::new(move || App::new().configure(app_config))
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
