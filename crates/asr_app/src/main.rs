use asr_core::config::ProdConfig;
use asr_infra::{DbPool};
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


#[tokio::main]
async fn main() {
  dotenv().ok();
  let _gaurd = tracing_init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");

  let db_pool = asr_infra::create_pool(&cfg).await.expect("Failed to create DB pool");
  let db_state = Arc::new(db_pool);


  let app =  Router::new()
      .merge(asr_api::user_routes())
      .layer(middleware::map_response(map_response::mw_map_response)) // 1
      .layer(middleware::from_fn_with_state(db_state.clone(), mw_auth::mw_auth)) // 2
      .with_state(db_state.clone());

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  info!("Server is running on port: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}



