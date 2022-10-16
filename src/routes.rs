use actix_web::web::ServiceConfig;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod ping;

pub struct WebappData {
  pub db_connection: Pool<ConnectionManager<PgConnection>>,
}

pub fn initialise(cfg: &mut ServiceConfig) {
  cfg.service((
    // Put the routes of your application here in this tuple
    ping::ping,
  ));
}
