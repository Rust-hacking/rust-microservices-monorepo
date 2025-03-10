use asr_core::config::ProdConfig;
use asr_infra::{initialed_db, connect_to_sql_server, DbPool};
use asr_infra::middleware::{map_response, mw_auth};
mod trace;
use trace::tracing_init;
use tiberius::{ AuthMethod, Client, Config};
use chrono::{NaiveDateTime};
use std::sync::Arc;

use axum::{
  middleware::{self},
  Router,
  extract::State, Json,
  routing::get
};
use dotenv::dotenv;
use tracing::info;

#[derive(serde::Serialize)]
struct User {
  id: i32,
  name: String,
}

async fn get_users(State(pool): State<Arc<DbPool>>) -> Json<Vec<User>> {
  let mut conn = pool.get().await.expect("Failed to get DB connection");

  let query = "SELECT id, name FROM Users";
  let rows = conn.simple_query(query).await.unwrap().into_first_result().await.unwrap();

  let users: Vec<User> = rows.into_iter().map(|row| User {
    id: row.get::<i32, _>(0).unwrap(),
    name: row.get::<&str, _>(1).unwrap().to_string(),
  }).collect();

  Json(users)
}


#[tokio::main]
async fn main() {
  dotenv().ok();
  let _gaurd = tracing_init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");

  let db_pool = asr_infra::create_pool(&cfg).await.expect("Failed to create DB pool");
  let db_state = Arc::new(db_pool);


  let app = Router::new().route("/users", get(get_users)).with_state(db_state.clone());

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  info!("Server is running on port: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}


async fn check(cfg: &ProdConfig) -> Result<(), Box<dyn std::error::Error>> {
  let mut client = connect_to_sql_server(&cfg).await?;

  let query = "SELECT id, name, email, created_at FROM users";
  let rows = client.simple_query(query).await?.into_results().await?;

  // Print results
  for row in rows[0].iter() {
    let id: i32 = row.get("id").unwrap();
    let name: &str = row.get("name").unwrap();
    let email: &str= row.get("email").unwrap();
    let created_at: NaiveDateTime = row.get("created_at").unwrap();

    println!("ID: {}, Name: {}, Email: {}, Created At: {}", id, name, email, created_at);
  }

  Ok(())
}


