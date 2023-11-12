mod error;
mod utils;

#[cfg(not(debug_assertions))]
use serde_json::Value;
#[cfg(not(debug_assertions))]
use std::cell::OnceCell;

use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use askama::Template;
pub use axum::*;
use axum::{
    handler::HandlerWithoutStateExt,
    response::{Html, IntoResponse, Response as AxumResponse},
    routing::get,
};
use config::Config;
use database::DB;
use prefork::{Prefork, DEFAULT_NUM_PROCESSES};
use tokio::runtime::Builder;
use tower_http::services::ServeDir;

pub use crate::error::{panic_handler, Error};
pub use crate::utils::*;

pub type Result<T> = std::result::Result<T, Error>;

pub struct State {
    pub db: DB,
}

#[cfg(debug_assertions)]
pub const PRODUCTION: bool = false;

#[cfg(not(debug_assertions))]
pub const PRODUCTION: bool = true;

#[cfg(not(debug_assertions))]
pub const MANIFEST: OnceCell<Value> = OnceCell::new();

#[cfg(not(debug_assertions))]
fn get_manifest() -> Value {
    let manifest_path = std::path::Path::new("public/manifest.json");
    let manifest = std::fs::read_to_string(manifest_path).unwrap();
    serde_json::from_str(&manifest).unwrap()
}

pub type AppState = Arc<State>;

pub type Router = axum::Router<AppState>;

pub type Response<T> = Result<T>;

pub struct System {
    address: SocketAddr,
    prefork: u32,
    router: Router,
    config_path: String,
    config: Config,
    db: Option<DB>,
}

impl State {
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
            prefork: 1,
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

    pub fn prefork(mut self, num_process: u32) -> Self {
        self.prefork = num_process;
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

    pub fn set_prefork(&mut self, num_process: u32) -> &mut Self {
        self.prefork = num_process;
        self
    }

    pub fn set_db(&mut self, db: DB) -> &mut Self {
        self.db = Some(db);
        self
    }

    async fn create_state(&self) -> AppState {
        let mut config = self.config.clone();

        if !self.config_path.is_empty() {
            config = Config::load_config_file(&self.config_path);
        }

        let db = match self.db.clone() {
            Some(db) => db,
            None => match DB::connect(&config.database.to_database_url()).await {
                Ok(db) => db,
                Err(e) => {
                    panic!("Failed to connect to database: {}", e);
                }
            },
        };

        Arc::new(State { db })
    }

    async fn server(&self, listener: TcpListener) -> Result<()> {
        async fn not_found() -> Error {
            Error::PageNotFound
        }

        let public_dir = ServeDir::new("public").not_found_service(not_found.into_service());

        Server::from_tcp(listener)
            .map_err(|_| Error::FailedToStartServer)?
            .serve(
                self.router
                    .clone()
                    .with_state(self.create_state().await)
                    .fallback_service(public_dir)
                    .into_make_service(),
            )
            .await
            .map_err(|_| Error::FailedToStartServer)?;

        Ok(())
    }

    pub fn run(self) -> Result<()> {
        let listener = TcpListener::bind(self.address).expect("Failed to bind to address");
        if self.prefork == 1 {
            Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("cannot create runtime")
                .block_on(async {
                    match self.server(listener).await {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to start server: {}", e);
                            return;
                        }
                    }
                })
        } else {
            let num_processes = if self.prefork != 0 {
                self.prefork
            } else {
                DEFAULT_NUM_PROCESSES
            };
            if Prefork::from_resource((listener, self))
                .with_num_processes(num_processes)
                .with_init(|child_num, (listener, app)| {
                    Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .expect("cannot create runtime")
                        .block_on(async {
                            let pid = std::process::id();
                            println!("Child {} (PID {}) started", child_num, pid);
                            match app.server(listener).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to start server: {}", e);
                                    return;
                                }
                            }
                        })
                })
                .fork()
                .map_err(|_| Error::FailedToStartServer)?
            {
                println!("Parent is exiting");
            }
        }

        Ok(())
    }
}

pub fn asset(path: &str) -> String {
    #[cfg(debug_assertions)]
    return format!("http://localhost:5173/src/resources/assets{}", path);
    #[cfg(not(debug_assertions))]
    return get_asset_from_manifest(path);
}

#[cfg(not(debug_assertions))]
fn get_asset_from_manifest(path: &str) -> String {
    let binding = MANIFEST;
    let manifest = binding.get_or_init(get_manifest);
    match &manifest[format!("src/resources/assets{}", path)]["file"] {
        Value::String(s) => format!("/{}", s),
        _ => path.to_string(),
    }
}
