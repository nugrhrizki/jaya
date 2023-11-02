pub mod error;

use std::{collections::HashMap, time::Duration};

use error::Error;
use serde_json::{Map, Value};
pub use sqlx::*;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult, PgRow},
    Pool, Postgres,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}

pub struct Table {
    pub table_name: String,
    pub primary_key: String,
    pub fields: HashMap<String, Value>,
    pub row: Option<PgRow>,
}

pub trait Model: From<Table> {
    fn schema(&self, schema: Table) -> Table;
}

impl Table {
    pub fn new() -> Self {
        Table::default()
    }

    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();
        self
    }

    pub fn primary_key(mut self, primary_key: &str) -> Self {
        self.primary_key = primary_key.to_string();
        self
    }

    pub fn field(mut self, field: &str, value: Value) -> Self {
        self.fields.insert(field.to_string(), value);
        self
    }

    pub fn get_str(&self, field: &str) -> Option<&str> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_bool(&self, field: &str) -> Option<bool> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_i32(&self, field: &str) -> Option<i32> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_f32(&self, field: &str) -> Option<f32> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_i64(&self, field: &str) -> Option<i64> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_f64(&self, field: &str) -> Option<f64> {
        self.row.as_ref().and_then(|row| row.try_get(field).ok())
    }

    pub fn get_array(&self, field: &str) -> Option<Vec<Value>> {
        self.fields
            .get(field)
            .and_then(|v| v.as_array().map(|a| a.to_vec()))
    }

    pub fn get_object(&self, field: &str) -> Option<Map<String, Value>> {
        self.fields
            .get(field)
            .and_then(|v| v.as_object().map(|o| o.to_owned()))
    }

    pub fn get_value(&self, field: &str) -> Option<Value> {
        self.fields.get(field).map(|v| v.to_owned())
    }

    pub fn scan<T>(row: PgRow) -> T
    where
        T: From<Table>,
    {
        Table::from(row).into()
    }

    pub fn scans<T>(rows: Vec<PgRow>) -> Vec<T>
    where
        T: From<Table>,
    {
        rows.into_iter().map(|row| Table::scan(row)).collect()
    }
}

impl From<PgRow> for Table {
    fn from(row: PgRow) -> Self {
        let mut schema = Table::new();
        schema.row = Some(row);
        schema
    }
}

impl Default for Table {
    fn default() -> Self {
        Table {
            primary_key: "".to_string(),
            table_name: "".to_string(),
            fields: HashMap::new(),
            row: None,
        }
    }
}

impl DB {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DB { pool }
    }

    pub async fn connect(conn_str: &str) -> Result<Self> {
        Ok(DB {
            pool: PgPoolOptions::new()
                .acquire_timeout(Duration::from_secs(5))
                .idle_timeout(Some(Duration::from_secs(60)))
                .connect(conn_str)
                .await?,
        })
    }

    pub fn pool(mut self, pool: Pool<Postgres>) -> Self {
        self.pool = pool;
        self
    }

    pub fn set_pool(&mut self, pool: Pool<Postgres>) -> &mut Self {
        self.pool = pool;
        self
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn insert(&self, model: impl Model) -> Result<PgQueryResult> {
        let schema = model.schema(Table::new());
        let mut qb = QueryBuilder::new(format!("INSERT INTO {} (", schema.table_name));
        let mut values = Vec::new();
        let len = schema.fields.len();
        for (i, (field, value)) in schema.fields.iter().enumerate() {
            if field.to_owned() == schema.primary_key {
                continue;
            }

            qb.push(field);
            values.push(value);

            if i < len - 1 {
                qb.push(", ");
            } else {
                qb.push(") VALUES (");
            }
        }

        for (i, value) in values.iter().enumerate() {
            qb.push(value);

            if i < len - 1 {
                qb.push(", ");
            } else {
                qb.push(")");
            }
        }

        qb.build()
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Database(e))
    }

    pub async fn get<T>(&self, model: impl Model) -> Result<Vec<T>>
    where
        T: From<Table>,
    {
        let schema = model.schema(Table::new());

        sqlx::query(format!("SELECT * FROM {}", schema.table_name).as_str())
            .fetch_all(&self.pool)
            .await
            .map(|rows| Table::scans(rows))
            .map_err(|e| Error::Database(e))
    }

    pub async fn find<T>(&self, model: impl Model) -> Result<T>
    where
        T: From<Table>,
    {
        let schema = model.schema(Table::new());
        let mut qb = QueryBuilder::new(format!("SELECT * FROM {} WHERE ", schema.table_name));
        let primary_key = schema.primary_key;
        let primary_value = schema.fields.get(&primary_key).unwrap();
        qb.push(primary_key);
        qb.push(" = ");
        qb.push_bind(primary_value);

        qb.build()
            .fetch_one(&self.pool)
            .await
            .map(|row| Table::from(row).into())
            .map_err(|e| Error::Database(e))
    }
}
