use letsgetrusty::configuration::get_configuration;
use letsgetrusty::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;
 
#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("{:?}", configuration.database.connection_string());
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Connected");
    run(listener, connection_pool)?.await
}
