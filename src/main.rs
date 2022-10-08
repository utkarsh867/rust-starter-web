use std::env;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger;

pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "debug");
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  let port = match env::var("port") {
    Ok(port) => port.parse::<u16>().unwrap(),
    Err(_) => 4000
  };

  HttpServer::new(|| {
    App::new()
    .configure(routes::initialise)
    .wrap(Logger::new("%t %a %r %s %{User-Agent}i"))
  }).bind((String::from("0.0.0.0"), port))?
    .run()
    .await
}
