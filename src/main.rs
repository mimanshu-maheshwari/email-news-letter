use std::net::TcpListener;

use enl::{configuration, startup, telemetry};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("enl".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPoolOptions::new() 
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret());

    // let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret()).expect("Failed to connect to Postgres pool.");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}

