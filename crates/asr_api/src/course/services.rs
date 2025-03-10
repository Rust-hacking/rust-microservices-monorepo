use axum::{extract::State, Json};
use asr_core::AppResult;
use asr_domain::user::{RequestCreateCourse, ResponseCreateCourse};
use sqlx::PgPool;

use super::CourseDmc;

impl CourseDmc {
  pub async fn create_course(
    State(db): State<PgPool>,
    Json(req): Json<RequestCreateCourse>,
  ) -> AppResult<Json<ResponseCreateCourse>> {
    asr_infra::user::create::<CourseDmc, _, _>(db, req).await
  }
}
