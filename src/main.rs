use std::net::TcpListener;

use enl::{configuration, startup, telemetry};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("enl".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres pool.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}
