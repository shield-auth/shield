use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, fmt};

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().expect("Failed to setup settings"));

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Admin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    pub signing_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub admin: Admin,
    pub secrets: Secrets,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        // Some cloud services like Heroku exposes a randomly assigned port in
        // the PORT env var and there is no way to change the env var name.
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        if let Ok(database_uri) = env::var("DATABASE_URL") {
            builder = builder.set_override("database.uri", database_uri)?;
        }
        if let Ok(admin_email) = env::var("ADMIN_USERNAME") {
            builder = builder.set_override("admin.email", admin_email)?;
        }
        if let Ok(admin_password) = env::var("ADMIN_PASSWORD") {
            builder = builder.set_override("admin.password", admin_password)?;
        }

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "http://localhost:{}", &self.port)
    }
}
