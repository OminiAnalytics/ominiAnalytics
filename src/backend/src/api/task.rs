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

use derive_more::Display;
use serde::{Deserialize, Serialize};

// Diesel stuff
extern crate diesel;
use crate::{DBPool, DBPooledConnection};
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use std::str::FromStr;

// Import models
use crate::models::TaskDB;
use crate::schema::tasks;

#[derive(Serialize, Deserialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub content: String,
}

impl Task {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }

    pub fn to_task_db(&self) -> TaskDB {
        TaskDB {
            name: self.name.clone(),
            content: self.content.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub status: String,
}

#[get("/tasks/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>, pool: Data<DBPool>) -> Json<String> {
    let conn = pool.get().expect("Connection error");
    let id = String::from(task_identifier.task_global_id.as_str()).parse::<i32>();
    if id.is_err() {
        return Json(String::from("Invalid task id"));
    } else {
        let mut id = id.unwrap();
        id += 1;
        Json(id.to_string())
    }
}

pub fn post_task_to_db(task: Task, conn: &DBPooledConnection) -> Result<Task, Error> {
    use crate::schema::tasks::dsl::*;
    let _ = diesel::insert_into(tasks)
        .values(task.to_task_db())
        .execute(conn);

    Ok(task)
}

#[post("/tasks")]
pub async fn post_task(pool: Data<DBPool>, body: Json<Task>) -> Json<Response> {
    let conn = pool.get().expect("Connection error");
    let task = Task::new(body.name.clone(), body.content.clone());
    let query = post_task_to_db(task, &conn);
    if query.is_err() {
        let r = Response {
            message: String::from("Error creating task"),
            status: String::from("error"),
        };
        return Json(r);
    } else {
        let r = Response {
            message: String::from("Task created"),
            status: String::from("success"),
        };
        return Json(r);
    }
}
