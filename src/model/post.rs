use ramhorns::Content;
use serde::{Deserialize, Serialize};

use database::{Model, Table};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Content)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Model for Post {
    fn schema(&self, schema: Table) -> Table {
        schema
            .table_name("post")
            .field("id", Value::Number(self.id.into()))
            .field("title", Value::String(self.title.clone()))
            .field("body", Value::String(self.body.clone()))
            .field("published", Value::Bool(self.published))
    }
}

impl From<Table> for Post {
    fn from(schema: Table) -> Self {
        Post {
            id: schema.get_i64("id").unwrap_or(0) as i32,
            title: schema.get_str("title").unwrap_or("").to_string(),
            body: schema.get_str("body").unwrap_or("").to_string(),
            published: schema.get_bool("published").unwrap_or(false),
        }
    }
}
