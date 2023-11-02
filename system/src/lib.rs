pub mod error;

use std::{net::SocketAddr, sync::Arc};

use crate::error::Error;
use askama::Template;
pub use axum::*;
use axum::{
    handler::HandlerWithoutStateExt,
    response::{Html, IntoResponse, Response as AxumResponse},
    routing::get,
};
use config::Config;
use database::DB;
use tower_http::services::ServeDir;

pub type Result<T> = std::result::Result<T, Error>;

pub struct SystemState {
    pub db: DB,
}

pub type State = axum::extract::State<Arc<SystemState>>;

pub type Router = axum::Router<Arc<SystemState>>;

pub type Response<T> = Result<T>;

pub struct System {
    address: SocketAddr,
    router: Router,
    config_path: String,
    config: Config,
    db: Option<DB>,
}

impl SystemState {
    pub fn render<T>(&self, template: T) -> AxumResponse
    where
        T: Template,
    {
        match template.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Error::TemplateError(err).into_response(),
        }
    }
}

impl Default for System {
    fn default() -> Self {
        Self {
            address: "0.0.0.0:3000".parse().unwrap(),
            router: Router::new().route("/", get(|| async { "Hello, World!" })),
            config_path: "".to_string(),
            config: Config::default(),
            db: None,
        }
    }
}

impl System {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_router(router: Router) -> Self {
        System {
            router,
            ..Default::default()
        }
    }

    pub fn with_config(config: Config) -> Self {
        System {
            config,
            ..Default::default()
        }
    }

    pub fn with_db(db: DB) -> Self {
        System {
            db: Some(db),
            ..Default::default()
        }
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub fn config_path(mut self, config_path: &str) -> Self {
        self.config_path = config_path.to_string();
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn db(mut self, db: DB) -> Self {
        self.db = Some(db);
        self
    }

    pub fn address(mut self, address: SocketAddr) -> Self {
        self.address = address;
        self
    }

    pub fn set_router(&mut self, router: Router) -> &mut Self {
        self.router = router;
        self
    }

    pub fn set_config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn set_config_path(&mut self, config_path: &str) -> &mut Self {
        self.config_path = config_path.to_string();
        self
    }

    pub fn set_db(&mut self, db: DB) -> &mut Self {
        self.db = Some(db);
        self
    }

    pub async fn run(&self) -> Result<()> {
        async fn not_found() -> Result<()> {
            Err(Error::PageNotFound)
        }

        let public_dir = ServeDir::new("public").not_found_service(not_found.into_service());

        let mut config = self.config.clone();

        if !self.config_path.is_empty() {
            config = Config::load_config_file(&self.config_path);
        }

        let db = match self.db.clone() {
            Some(db) => db,
            None => DB::connect(&config.database.to_database_url())
                .await
                .map_err(|e| Error::Database(e))?,
        };

        let state = Arc::new(SystemState { db });

        Server::bind(&self.address)
            .serve(
                self.router
                    .clone()
                    .with_state(state)
                    .fallback_service(public_dir)
                    .into_make_service(),
            )
            .await
            .map_err(|_| Error::FailedToStartServer)?;

        Ok(())
    }
}
