use std::env;
use actix_web::{App, HttpServer};

pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port = match env::var("port") {
    Ok(port) => port.parse::<u16>().unwrap(),
    Err(_) => 4000
  };

  HttpServer::new(|| {
    App::new().configure(routes::initialise)
  }).bind((String::from("0.0.0.0"), port))?
    .run()
    .await
}
