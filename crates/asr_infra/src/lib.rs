pub mod middleware;
pub mod user;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tiberius::{ AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures_util::stream::TryStreamExt;
use asr_core::config::ProdConfig;
use std::sync::Arc;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::env;

pub type DbPool = Pool<ConnectionManager>;


pub async fn initialed_db(dsn: &str, max_conns: u32) -> PgPool {
  let db = PgPoolOptions::new().max_connections(max_conns).connect(dsn).await.expect("Cannot connect database");

  sqlx::migrate!().run(&db).await.expect("Cannot migrate database");

  db
}


pub async fn create_pool(db_config: &ProdConfig) -> Result<DbPool, Box<dyn std::error::Error>> {
  let mut config = Config::new();
  config.host(&db_config.db.host);  // Change to your server
  config.port(db_config.db.port);         // Default SQL Server port
  config.authentication(AuthMethod::sql_server(&db_config.db.username, &db_config.db.password));
  config.database(&db_config.db.database);
  config.trust_cert();

  let manager = ConnectionManager::new(config);
  let pool = Pool::builder().max_size(5).build(manager).await?;

  Ok(pool)
}

pub async fn connect_to_sql_server(db_config: &ProdConfig) -> Result<Client<tokio_util::compat::Compat<TcpStream>>, Box<dyn std::error::Error>> {
  let mut config = Config::new();
  config.host(&db_config.db.host);  // Change to your server
  config.port(db_config.db.port);         // Default SQL Server port
  config.authentication(AuthMethod::sql_server(&db_config.db.username, &db_config.db.password));
  config.database(&db_config.db.database);
  config.trust_cert();

  let tcp = TcpStream::connect(config.get_addr()).await?;
  tcp.set_nodelay(true)?;

  let client = Client::connect(config, tcp.compat_write()).await?;
  println!("Connected to SQL Server!");

  Ok(client)
}
