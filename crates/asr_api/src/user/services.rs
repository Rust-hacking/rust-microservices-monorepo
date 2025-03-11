use std::sync::Arc;
use axum::{
  extract::{Path, State},
  Json,
};
use asr_core::AppResult;
use asr_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  User,
};
use asr_infra::DbPool;
use super::UserDmc;

impl UserDmc {
  // pub async fn create_user(State(db): State<Arc<DbPool>>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<User>> {
  //   asr_infra::user::create::<UserDmc, _, _>(db, req).await
  // }

  pub async fn get_user_by_id(State(db): State<Arc<DbPool>>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    asr_infra::user::get_user(db, id).await
  }

  pub async fn get_users(State(db): State<Arc<DbPool>>) -> AppResult<Json<Vec<User>>> {
    asr_infra::user::list(db).await
  }

  pub async fn update_user(State(db): State<Arc<DbPool>>, Json(req): Json<RequestUpdateUser>) -> AppResult<()> {
    asr_infra::user::update(db, req).await
  }

  pub async fn delete_user(State(db): State<Arc<DbPool>>, Path(req): Path<i32>) -> AppResult<()> {
    asr_infra::user::delete(db, req).await
  }
}
