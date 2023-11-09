use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: Database,
}

#[derive(Deserialize, Clone)]
pub struct Database {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: Default::default(),
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            name: "jaya".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
        }
    }
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    pub fn to_connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.username, self.password, self.name
        )
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_config_file(_path: &str) -> Self {
        Self::default()
    }
}

pub struct Menu {
    pub name: &'static str,
    pub url: &'static str,
    pub icon: &'static str,
}

pub const MENU: [Menu; 3] = [
    Menu {
        name: "Dashboard",
        url: "/",
        icon: "lucide:home",
    },
    Menu {
        name: "Post",
        url: "/post",
        icon: "lucide:file-text",
    },
    Menu {
        name: "User",
        url: "/user",
        icon: "lucide:user",
    },
];
