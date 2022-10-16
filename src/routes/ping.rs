use crate::models;
use crate::models::User;
use crate::WebappData;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/ping")]
pub async fn ping(data: Data<WebappData>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let mut pool_connection = data
        .db_connection
        .get()
        .expect("Could not get a pooled connection");
    let users_from_db = web::block(move || users.load::<User>(&mut pool_connection))
        .await
        .expect("Cannot make user query on the database.")
        .unwrap();

    for user in users_from_db {
        println!("{}", user.username);
    }

    HttpResponse::Ok().body("OK")
}
