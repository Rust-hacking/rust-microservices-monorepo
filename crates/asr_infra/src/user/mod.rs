use std::sync::Arc;
use axum::Json;
use asr_core::{error::AppError, AppResult};
use asr_domain::user::{
  request::{RequestGetUser, RequestUpdateUser},
  User,
};
use modql::{field::HasFields, SIden};
use sea_query::{Alias, IntoIden, PostgresQueryBuilder, Query, TableRef};
use crate::DbPool;

pub trait DMC {
  const SCHEMA: &'static str;
  const TABLE: &'static str;

  fn table_ref() -> TableRef {
    TableRef::SchemaTable(SIden(Self::SCHEMA).into_iden(), SIden(Self::TABLE).into_iden())
  }
}

// DMC -> Database Model Control
// pub async fn create<MC, I, O>(db: Arc<DbPool>, input: I) -> AppResult<Json<O>>
// where
//   MC: DMC,
//   I: HasFields,
//   O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
// {
//   let mut conn = db.get().await.expect("Failed to get DB connection");
//
//   let query = "SELECT id, name FROM Users";
//   let rows = conn.simple_query(query).await.unwrap().into_first_result().await.unwrap();
//
//   let users: Vec<User> = rows.into_iter().map(|row| User {
//     pk_user_id: row.get::<i64, _>(0).unwrap(),
//     username: row.get::<&str, _>(1).unwrap().to_string(),
//   }).collect();
//
//   Ok(Json(users.collect()))
// }

pub async fn get_user(db: Arc<DbPool>, id: RequestGetUser) -> AppResult<Json<User>> {
  let mut conn = db.get().await.expect("Failed to get DB connection");

  let query = "SELECT id, name FROM Users";
  let rows = conn.simple_query(query).await.unwrap().into_first_result().await.unwrap();

  let users: Vec<User> = rows.into_iter().map(|row| User {
    id: row.get::<i32, _>(0).unwrap(),
    name: row.get::<&str, _>(1).unwrap().to_string(),
  }).collect();

  Ok(Json(users.into_iter().next().ok_or(AppError::NotFound)?))
}

pub async fn list(db: Arc<DbPool>) -> AppResult<Json<Vec<User>>> {

  let mut conn = db.get().await.expect("Failed to get DB connection");

  let query = "SELECT id, name FROM Users";
  let rows = conn.simple_query(query).await.unwrap().into_first_result().await.unwrap();

  let users: Vec<User> = rows.into_iter().map(|row| User {
    id: row.get::<i32, _>(0).unwrap(),
    name: row.get::<&str, _>(1).unwrap().to_string(),
  }).collect();

  Ok(Json(users))
}

pub async fn update(db: Arc<DbPool>, req: RequestUpdateUser) -> AppResult<()> {
  Ok(())
}

pub async fn delete(db: Arc<DbPool>, id: i32) -> AppResult<()> {

  Ok(())
}
