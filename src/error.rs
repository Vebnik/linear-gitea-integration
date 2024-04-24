use actix_web::{
    error::ResponseError,
    http::header::ContentType,
    HttpResponse,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("Database execute error")]
    DbError,

    #[error("TCP connect")]
    TcpError,

    #[error("Undefined behavior")]
    UbError,

    #[error("Undefined behavior: {0}")]
    UbExtendError(String),

    // #[error("Not create tcp connection with client")]
    // ClientNotCreate,
}

/// A convenience type alias for `Result<T, Error>`.
pub type Result<T, E = CustomError> = anyhow::Result<T, E>;

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({"message": self.to_string()}).to_string())
    }
}
