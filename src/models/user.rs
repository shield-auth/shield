use crate::database::user::Model;

impl Model {
    pub fn verify_password(&self, password: &str) -> bool {
        match self.password_hash {
            Some(ref hash) => bcrypt::verify(password, hash).unwrap_or(false),
            None => false,
        }
    }
}
