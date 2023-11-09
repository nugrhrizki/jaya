use std::any::Any;

use askama::Template;
use axum::{
    body::Body,
    http::{self, header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Http(axum::Error),
    Database(sqlx::Error),
    FailedToStartServer,
    TemplateError(askama::Error),
    Panic(String),
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
            Error::Panic(e) => write!(f, "{}", e),
            Error::TemplateError(e) => write!(f, "{}", e),
        }
    }
}

struct ErrorDetails {
    kind: Error,
    details: String,
}

impl std::fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    code: http::StatusCode,
    error: ErrorDetails,
}

impl ErrorTemplate {
    pub fn new(code: http::StatusCode, error: ErrorDetails) -> Self {
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
            Error::PageNotFound => ErrorTemplate::new(http::StatusCode::NOT_FOUND, {
                ErrorDetails {
                    kind: self,
                    details: "".to_string(),
                }
            })
            .into_response(),
            _ => ErrorTemplate::new(http::StatusCode::INTERNAL_SERVER_ERROR, {
                ErrorDetails {
                    kind: self,
                    details: "".to_string(),
                }
            })
            .into_response(),
        }
    }
}

pub fn panic_handler(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    let body = ErrorTemplate::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorDetails {
            kind: Error::Panic("Panic".to_string()),
            details,
        },
    )
    .render()
    .expect("Failed to render error template");

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body(Body::from(body))
        .unwrap()
}
