use std::env;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = env::current_dir().expect("Current directory should be gotten");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Invalid or absent APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            unknown => Err(format!(
                "{unknown} is not a supported environment. \"
                Use either `local` or `production`."
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn format_connection_string(&self) -> String {
        let connection_string_without_db = self.format_connection_string_without_db();
        let connection_string_without_db = connection_string_without_db.expose_secret();
        let database_name = &self.name;

        format!("{connection_string_without_db}/{database_name}")
    }

    pub fn format_connection_string_without_db(&self) -> SecretString {
        let Self {
            username,
            password,
            host,
            port,
            name: _,
        } = self;
        let password = password.expose_secret();

        format!("postgres://{username}:{password}@{host}:{port}").into()
    }
}
