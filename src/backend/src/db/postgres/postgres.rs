use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::Client;

use crate::db::models::model::Task;

pub async fn add_task(client: &Client, task: Task) -> std::io::Result<()> {}
