use std::sync::Arc;
use axum::Router;
use asr_infra::DbPool;

mod course;
mod user;

pub fn user_routes() -> Router<Arc<DbPool>> {
  Router::new().nest("/api/v1", Router::new().merge(user::routes()))
}
