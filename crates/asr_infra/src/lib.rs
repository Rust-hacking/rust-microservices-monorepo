use tiberius::{ AuthMethod, Client, Config};
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures_util::stream::TryStreamExt;
use asr_core::config::ProdConfig;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;

pub mod middleware;
pub mod user;
pub type DbPool = Pool<ConnectionManager>;


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
