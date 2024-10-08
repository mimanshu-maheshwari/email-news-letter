use config::ConfigError;
use secrecy::{Secret, ExposeSecret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub port: u16, 
    pub host: String,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require, 
            false => PgSslMode::Prefer,
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

pub enum Environment {
    Local, 
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local", 
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not supported environment. Use either `local` or `production`.", other)),
        }
    }
    
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    // let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let settings = config::Config::builder()
        .add_source(config::File::from(
                configuration_dir.join("base"))
            .required(true))
        .add_source(config::File::from(
                configuration_dir.join(environment.as_str()))
            .required(true))
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()
        .expect("Failed to read configuration from file.");

    // Try to convert the configuration values it read into
    let settings: Result<Settings, ConfigError> = settings.try_deserialize();
    settings
}
