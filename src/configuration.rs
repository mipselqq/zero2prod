use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub name: String,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
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
