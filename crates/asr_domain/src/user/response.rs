use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseCreateUser {
  pub id: i32,
  pub name: String,
  pub email: String,
}
