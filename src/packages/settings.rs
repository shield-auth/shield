use crate::packages::errors::Error;
use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::Deserialize;
use std::{env, fmt, fs::read_to_string, sync::Arc};

use crate::utils::helpers::default_cred::DefaultCred;

// pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().expect("Failed to setup settings"));
pub static SETTINGS: Lazy<Arc<RwLock<Settings>>> = Lazy::new(|| Arc::new(RwLock::new(Settings::new().expect("Failed to setup settings"))));

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
    pub default_cred: DefaultCred,
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

        let default_cred_str = read_to_string("./logs/default_cred.txt").map_err(Error::from);
        let default_cred_str = default_cred_str.unwrap();
        let default_cred = DefaultCred::from_str(&default_cred_str);
        let default_cred = default_cred.unwrap();
        builder = builder.set_override("default_cred.realm_id", default_cred.realm_id.to_string())?;
        builder = builder.set_override("default_cred.client_id", default_cred.client_id.to_string())?;

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }

    pub fn reload() -> Result<(), ConfigError> {
        let new_settings = Settings::new()?;
        let mut settings = SETTINGS.write();
        *settings = new_settings;
        Ok(())
    }

    pub fn get<F, T>(f: F) -> T
    where
        F: FnOnce(&Settings) -> T,
    {
        let settings = SETTINGS.read();
        f(&settings)
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "http://localhost:{}", &self.port)
    }
}
