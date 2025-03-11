use asr_infra::user::DMC;

mod routes;
mod services;

pub use routes::routes;

pub struct UserDmc;

impl DMC for UserDmc {
  const SCHEMA: &'static str = "learning";
  const TABLE: &'static str = "tbl_users";
}
