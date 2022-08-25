use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "tasks")] // singular 'user' is a keyword..
pub struct Task {
    pub name: String,
    pub content: String,
}
