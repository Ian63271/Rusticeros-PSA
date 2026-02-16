use axum::{
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

use shared::{Task, TaskResult};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/task", get(get_task))
        .route("/result", post(receive_result));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Coordinator escuchando en http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}

// GET /task
async fn get_task() -> Json<Task> {
    let dummy_task = Task {
        id: 1,
        payload: "calcular_algo_dummy".to_string(),
    };

    println!("Enviando tarea dummy");

    Json(dummy_task)
}

// POST /result
async fn receive_result(Json(result): Json<TaskResult>) -> String {
    println!("Resultado recibido: {:?}", result);

    "Resultado recibido correctamente".to_string()
}

