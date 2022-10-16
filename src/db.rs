use std::env;
use diesel::{Connection, pg, PgConnection};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_connection_pool() -> diesel::r2d2::Pool<ConnectionManager<PgConnection>> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  let connection_manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
  Pool::builder().build(connection_manager).expect("Could not create a connection pool")
}

pub fn establish_connection() -> PgConnection {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
