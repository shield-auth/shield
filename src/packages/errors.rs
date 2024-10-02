use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde_json::json;
use std::io;
use tokio::task::JoinError;
use tracing::error;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequestError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),

    #[error("{0}")]
    FileError(#[from] FileError),

    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match self {
            // 4XX Errors
            Error::BadRequest(err) => err.get_codes(),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            Error::Authenticate(err) => err.get_codes(),

            // 5XX Errors
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
            Error::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5007),
            Error::FileError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5008),
            Error::SerdeJson(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5009),
        }
    }

    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequestError::Generic)
    }

    pub fn not_found() -> Self {
        Error::NotFound(NotFound::Generic)
    }

    pub fn db_error<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Error::DatabaseError(DatabaseError(Box::new(err)))
    }

    pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

    pub fn invalid_input(message: &str) -> Self {
        Error::BadRequest(BadRequestError::InvalidInput(message.to_string()))
    }

    pub fn missing_field(field: &str) -> Self {
        Error::BadRequest(BadRequestError::MissingField(field.to_string()))
    }

    pub fn cannot_perform_operation(message: &str) -> Self {
        Error::BadRequest(BadRequestError::CannotPerformOperation(message.to_string()))
    }

    pub fn context(self, context: &str) -> Self {
        match self {
            Error::BadRequest(err) => Error::BadRequest(err.with_context(context)),
            Error::NotFound(err) => Error::NotFound(err.with_context(context)),
            Error::Authenticate(err) => Error::Authenticate(err.with_context(context)),
            _ => self,
        }
    }

    pub fn log(&self) {
        let (status, code) = self.get_codes();
        error!("Error {}/{}: {}", status.as_u16(), code, self);
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.log();
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Max concurrent sessions reached")]
    MaxConcurrentSessions,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("Invalid api credentials")]
    InvalidApiCredentials,
    #[error("Inappropriate resource access")]
    NoResource,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Action forbidden")]
    ActionForbidden,
    #[error("User is locked")]
    Locked,
}

impl AuthenticateError {
    fn get_codes(&self) -> (StatusCode, u16) {
        match self {
            AuthenticateError::WrongCredentials => (StatusCode::UNAUTHORIZED, 40004),
            AuthenticateError::InvalidToken => (StatusCode::UNAUTHORIZED, 40005),
            AuthenticateError::Locked => (StatusCode::LOCKED, 40006),
            AuthenticateError::EmailNotVerified => (StatusCode::FORBIDDEN, 40007),
            AuthenticateError::NoResource => (StatusCode::FORBIDDEN, 40008),
            AuthenticateError::ActionForbidden => (StatusCode::FORBIDDEN, 40009),
            AuthenticateError::MaxConcurrentSessions => (StatusCode::LOCKED, 40010),
            AuthenticateError::InvalidApiCredentials => (StatusCode::FORBIDDEN, 40011),
            AuthenticateError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, 5001),
        }
    }

    fn with_context(self, _context: &str) -> Self {
        match self {
            AuthenticateError::WrongCredentials => AuthenticateError::WrongCredentials,
            AuthenticateError::TokenCreation => AuthenticateError::TokenCreation,
            AuthenticateError::InvalidToken => AuthenticateError::InvalidToken,
            AuthenticateError::NoResource => AuthenticateError::NoResource,
            AuthenticateError::EmailNotVerified => AuthenticateError::EmailNotVerified,
            AuthenticateError::ActionForbidden => AuthenticateError::ActionForbidden,
            AuthenticateError::MaxConcurrentSessions => AuthenticateError::MaxConcurrentSessions,
            AuthenticateError::InvalidApiCredentials => AuthenticateError::InvalidApiCredentials,
            AuthenticateError::Locked => AuthenticateError::Locked,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BadRequestError {
    #[error("Bad Request")]
    Generic,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Cannot perform operation: {0}")]
    CannotPerformOperation(String),
}

impl BadRequestError {
    fn get_codes(&self) -> (StatusCode, u16) {
        (StatusCode::BAD_REQUEST, 40002)
    }

    fn with_context(self, context: &str) -> Self {
        match self {
            BadRequestError::Generic => BadRequestError::CannotPerformOperation(context.to_string()),
            BadRequestError::InvalidInput(msg) => BadRequestError::InvalidInput(format!("{}: {}", context, msg)),
            BadRequestError::MissingField(field) => BadRequestError::MissingField(format!("{}: {}", context, field)),
            BadRequestError::CannotPerformOperation(msg) => BadRequestError::CannotPerformOperation(format!("{}: {}", context, msg)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NotFound {
    #[error("Not found")]
    Generic,
    #[error("Resource not found: {0}")]
    Resource(String),
}

impl NotFound {
    fn with_context(self, context: &str) -> Self {
        match self {
            NotFound::Generic => NotFound::Resource(context.to_string()),
            NotFound::Resource(resource) => NotFound::Resource(format!("{}: {}", context, resource)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Database error: {0}")]
pub struct DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>);

#[derive(thiserror::Error, Debug)]
#[error("File error: {0}")]
pub struct FileError(#[from] io::Error);

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Error::DatabaseError(DatabaseError(Box::new(err)))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::FileError(FileError(err))
    }
}
