use super::schema::tasks;

#[derive(Queryable, Insertable)]
#[table_name = "tasks"]
pub struct TaskDB {
    pub name: String,
    pub content: String,
}
