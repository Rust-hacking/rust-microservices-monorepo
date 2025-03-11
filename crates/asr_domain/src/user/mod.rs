use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod request;
pub mod response;

#[derive(Serialize, FromRow, Fields)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct Course {
  pub id: i32,
  pub title: String,
  pub description: String,
}

#[derive(Deserialize, FromRow, Fields)]
pub struct RequestCreateCourse {
  pub id: i32,
  pub title: String,
  pub description: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResponseCreateCourse {
  pub id: i32,
  pub title: String,
}
