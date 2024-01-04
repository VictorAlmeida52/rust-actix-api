use actix_web::{web::{
    scope,
    ServiceConfig,
    Json,
    Data,
    Path
}, get, post, HttpResponse, Responder, delete, put};
use actix_web::web::Query;
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::schema::{CreateTaskSchema, FilterOptions, UpdateTaskSchema};
use crate::model::TaskModel;

#[get("/healthchecker")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "API  is healthy!";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/tasks")]
async fn create_task(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel, "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.content.to_string()
    ).fetch_one(&data.db).await {
        Ok(task) => {
            let note_response = json!({
                "status": "success",
                "task": json!({
                    "task": task
                })
            });

            return HttpResponse::Ok().json(note_response);
        }

        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })
            );
        }
    }
}

#[get("/tasks")]
async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY created_at LIMIT $1 OFFSET $2",
        limit as i32, offset as i32
    ).fetch_all(&data.db).await {
        Ok(tasks) => {
            let json_response = json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });

            return HttpResponse::Ok().json(json_response);
        }

        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })
            );
        }
    }
}

#[get("/tasks/{id}")]
async fn get_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks WHERE id = $1",
        task_id
    ).fetch_one(&data.db).await {
        Ok(task) => {
            let json_response = json!({
                "status": "success",
                "task": task
            });

            return HttpResponse::Ok().json(json_response);
        }

        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })
            );
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "DELETE FROM tasks WHERE id = $1",
        task_id
    ).execute(&data.db).await {
        Ok(_) => {
            return HttpResponse::NoContent().finish();
        }

        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })
            );
        }
    }
}

#[put("/tasks/{id}")]
async fn update_task_by_id(path: Path<Uuid>, body: Json<UpdateTaskSchema>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks WHERE id = $1",
        task_id,
    ).fetch_one(&data.db).await {
        Ok(task) => {

            match sqlx::query_as!(
                TaskModel,
                "UPDATE tasks SET title = $1, content = $2 WHERE id = $3 RETURNING *",
                body.title.to_owned().unwrap_or(task.title),
                body.content.to_owned().unwrap_or(task.content),
                task_id,
            ).fetch_one(&data.db).await {
                Ok(task) => {
                    let response = json!({
                        "status": "success",
                        "task": task
                    });
                    return HttpResponse::Ok().json(response);
                }

                Err(e) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": format!("Could not update task. {:?}", e)
                    }))
                }
            }

        }

        Err(e) => {
            return HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": format!("Task not found with provided ID. {:?}", e)
            }))
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_check)
        .service(create_task)
        .service(get_all_tasks)
        .service(get_task_by_id)
        .service(delete_task_by_id)
        .service(update_task_by_id);

    conf.service(scope);
}