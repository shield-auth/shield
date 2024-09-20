use crate::packages::errors::Error;
use tokio::task;

pub async fn generate_password_hash<P>(password: P) -> Result<String, Error>
where
    P: AsRef<str> + Send + 'static,
{
    let cost = 4;
    task::spawn_blocking(move || bcrypt::hash(password.as_ref(), cost))
        .await
        .map_err(Error::RunSyncTask)?
        .map_err(Error::HashPassword)
}
