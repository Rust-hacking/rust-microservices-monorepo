use modql::field::Fields;
use serde::Deserialize;

#[derive(Deserialize, Fields)]
pub struct RequestGetUser {
  pub id: i32,
}

#[derive(Deserialize, Fields)]
pub struct RequestCreateUser {
  pub name: String,
  pub email: String
}

#[derive(Deserialize, Fields)]
pub struct RequestUpdateUser {
  pub id: i32,
  pub name: String,
  pub email: String
}
