pub mod middleware;
pub mod user;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tiberius::{ AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures_util::stream::TryStreamExt;

pub async fn initialed_db(dsn: &str, max_conns: u32) -> PgPool {
  let db = PgPoolOptions::new().max_connections(max_conns).connect(dsn).await.expect("Cannot connect database");

  sqlx::migrate!().run(&db).await.expect("Cannot migrate database");

  db
}

async fn connect_to_sql_server() -> Result<Client<tokio_util::compat::Compat<TcpStream>>, Box<dyn std::error::Error>> {
  let mut config = Config::new();
  config.host("localhost");  // Change to your server
  config.port(1433);         // Default SQL Server port
  config.authentication(AuthMethod::sql_server("sa", "Password&2025"));
  config.database("master");
  config.trust_cert();

  let tcp = TcpStream::connect(config.get_addr()).await?;
  tcp.set_nodelay(true)?;

  let client = Client::connect(config, tcp.compat_write()).await?;
  println!("Connected to SQL Server!");

  Ok(client)
}
