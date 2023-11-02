use askama::Template;
use axum::{
    http,
    response::{Html, IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Http(axum::Error),
    Database(database::error::Error),
    FailedToStartServer,
    TemplateError(askama::Error),
    PageNotFound,
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Error::Http(e)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::Http(e) => write!(f, "{}", e),
            Error::Database(e) => write!(f, "{}", e),
            Error::FailedToStartServer => write!(f, "Failed to start server"),
            Error::PageNotFound => write!(f, "Page not found"),
            Error::TemplateError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    code: http::StatusCode,
    error: Error,
}

impl ErrorTemplate {
    pub fn new(code: http::StatusCode, error: Error) -> Self {
        Self { code, error }
    }
}

impl IntoResponse for ErrorTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => (self.code, Html(html).into_response()).into_response(),
            Err(err) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::PageNotFound => {
                ErrorTemplate::new(http::StatusCode::NOT_FOUND, self).into_response()
            }
            _ => ErrorTemplate::new(http::StatusCode::INTERNAL_SERVER_ERROR, self).into_response(),
        }
    }
}
