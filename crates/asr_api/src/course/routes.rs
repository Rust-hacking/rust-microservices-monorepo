use std::sync::Arc;
use axum::{routing::post, Router};
use asr_infra::DbPool;
use super::CourseDmc;

// pub fn routes() -> Router<Arc<DbPool>> {
//   Router::new().route("/course", post(CourseDmc::create_course))
// }
