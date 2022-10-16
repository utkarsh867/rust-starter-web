extern crate core;

pub mod routes;
pub mod models;
pub mod schema;
pub mod db;

use std::env;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::web::Data;
use diesel::PgConnection;
use env_logger;
use crate::db::establish_connection;
use crate::routes::{WebappData};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "debug");
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  let port = match env::var("port") {
    Ok(port) => port.parse::<u16>().unwrap(),
    Err(_) => 4000
  };

  let db_connection = db::create_connection_pool();

  let app_data = Data::new(WebappData {
    db_connection,
  });

  HttpServer::new(move || {
    App::new()
    .app_data(app_data.clone())
    .configure(routes::initialise)
    .wrap(Logger::new("%t %a %r %s %{User-Agent}i"))
  }).bind((String::from("0.0.0.0"), port))?
    .run()
    .await
}
