use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use crate::{db::models::model::Task, db::postgres};
use deadpool_postgres::{Client, Pool};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/tasks/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String> {
    let id = String::from(task_identifier.task_global_id.as_str()).parse::<i32>();
    if id.is_err() {
        return Json(String::from("Invalid task id"));
    } else {
        let mut id = id.unwrap();
        id += 1;
        Json(id.to_string())
    }
}
