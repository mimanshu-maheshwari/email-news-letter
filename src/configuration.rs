use config::ConfigError;
use secrecy::{Secret, ExposeSecret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    // let mut settings = config::Config::default();
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .expect("Failed to read configuration from file.");

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with and extension
    // that  `config` knows how to parse: yaml, json, etc.

    // Try to convert the configuration values it read into
    let settings: Result<Settings, ConfigError> = settings.try_deserialize();
    settings
}
