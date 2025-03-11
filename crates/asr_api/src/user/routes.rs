use std::sync::Arc;
use axum::{
  routing::{get, patch, post},
  Router,
};
use asr_infra::DbPool;
use super::UserDmc;

pub fn routes() -> Router<Arc<DbPool>> {
  Router::new()
    // .route("/user", post(UserDmc::create_user)) // login, admin - DCL
    .route("/user/:id", get(UserDmc::get_user_by_id).delete(UserDmc::delete_user))
    .route("/users", get(UserDmc::get_users))
    .route("/user/:id", patch(UserDmc::update_user))
}
