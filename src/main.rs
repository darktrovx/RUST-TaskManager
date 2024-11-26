use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;



#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    completed: bool,
}


struct AppState {
    tasks: Mutex<Vec<Task>>,
}

async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

async fn add_task(task: web::Json<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let mut new_task = task.into_inner();
    new_task.id = Uuid::new_v4();
    tasks.push(new_task);
    HttpResponse::Created().finish()
}

async fn update_task(
    task_id: web::Path<Uuid>,
    updated_task: web::Json<Task>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == *task_id) {
        task.title = updated_task.title.clone();
        task.completed = updated_task.completed;
        HttpResponse::Ok().json(task)
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

async fn delete_task(task_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(pos) = tasks.iter().position(|task| task.id == *task_id) {
        tasks.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(add_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}