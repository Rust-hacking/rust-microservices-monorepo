use std::sync::Arc;
use axum::Json;
use asr_core::{error::AppError, AppResult};
use asr_domain::user::{
  request::{RequestGetUser, RequestUpdateUser},
  User,
};
use modql::{field::HasFields, SIden};
use sea_query::{Alias, IntoIden, PostgresQueryBuilder, Query, TableRef};
use asr_domain::user::request::RequestCreateUser;
use crate::DbPool;

pub trait DMC {
  const SCHEMA: &'static str;
  const TABLE: &'static str;

  fn table_ref() -> TableRef {
    TableRef::SchemaTable(SIden(Self::SCHEMA).into_iden(), SIden(Self::TABLE).into_iden())
  }
}

// DMC -> Database Model Control
pub async fn create(db: Arc<DbPool>, input: RequestCreateUser) -> AppResult<Json<User>> {
  let query = "INSERT INTO users (name, email) VALUES (@P1, @P2)".to_string();
  execute_query_and_fetch_user(&db, query, &[&input.name, &input.email]).await
}

pub async fn get_user(db: Arc<DbPool>, request: RequestGetUser) -> AppResult<Json<User>> {
  let query = "SELECT * FROM Users WHERE id = @P1".to_string();
  execute_query_and_fetch_user(&db, query, &[&request.id]).await
}

pub async fn list(db: Arc<DbPool>) -> AppResult<Json<Vec<User>>> {
  let query = "SELECT * FROM Users".to_string();

  let users: Vec<User> = execute_query_and_fetch_users(&db, query, &[]).await;
    Ok(Json(users))
}

async fn execute_query_and_fetch_user(
  db: &Arc<DbPool>,
  query: String,
  params: &[&(dyn tiberius::ToSql)],
) -> AppResult<Json<User>> {
  let users: Vec<User> = execute_query_and_fetch_users(&db, query, params).await;
  Ok(Json(users.into_iter().next().ok_or(AppError::NotFound)?))
}


async fn execute_query_and_fetch_users(
  db: &Arc<DbPool>,
  query: String,
  params: &[&(dyn tiberius::ToSql)],
) -> Vec<User> {
  let mut conn = db.get().await.expect("Failed to get DB connection");
  let rows = conn.query(query, params).await.unwrap().into_first_result().await.unwrap();
  let users: Vec<User> = rows
      .into_iter()
      .map(|row| User {
        id: row.get::<i32, _>(0).unwrap(),
        name: row.get::<&str, _>(1).unwrap().to_string(),
        email: row.get::<&str, _>(2).unwrap().to_string(),
      })
      .collect();

  users
}






//
// pub async fn update(db: Arc<DbPool>, req: RequestUpdateUser) -> AppResult<()> {
//   Ok(())
// }
//
// pub async fn delete(db: Arc<DbPool>, id: i32) -> AppResult<()> {
//
//   Ok(())
// }
