use actix_web::web::ServiceConfig;

mod ping;

pub fn initialise(cfg: &mut ServiceConfig) {
  cfg.service((
    // Put the routes of your application here in this tuple
    ping::ping,
  ));
}
