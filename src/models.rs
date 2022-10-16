use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String
}
