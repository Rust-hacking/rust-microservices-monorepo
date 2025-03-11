use std::sync::Arc;
use axum::{extract::State, Json};
use asr_core::AppResult;
use asr_domain::user::{RequestCreateCourse, ResponseCreateCourse,User};
use asr_infra::DbPool;
use super::CourseDmc;

// impl CourseDmc {
//   pub async fn create_course(
//     State(db): State<Arc<DbPool>>,
//     Json(req): Json<RequestCreateCourse>,
//   ) -> AppResult<Json<User>> {
//     asr_infra::user::create::<CourseDmc, _, _>(db, req).await
//     Ok()
//   }
// }
