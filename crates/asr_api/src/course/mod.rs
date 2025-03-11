mod routes;
mod services;

use asr_infra::user::DMC;
// pub use routes::routes;

// -->>> Region:: START  --->>>  Data Model Control
pub struct CourseDmc;

impl DMC for CourseDmc {
  const SCHEMA: &'static str = "learning";
  const TABLE: &'static str = "tbl_courses";
}
// <<<-- Region:: END    <<<---  Data Model Control
