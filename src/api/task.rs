use actix_web::{
    error::ResponderError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponde,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> Json<String> {
    return Json(task_identifier.into_inner().task_global_id);
}
