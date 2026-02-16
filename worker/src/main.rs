use reqwest::Client;
use shared::{Task, TaskResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Solicitando tarea...");

    // GET /task
    let task: Task = client
        .get("http://localhost:3000/task")
        .send()
        .await?
        .json()
        .await?;

    println!("Tarea recibida: {:?}", task);

    // Simulamos procesamiento
    let result = TaskResult {
        task_id: task.id,
        result: format!("procesado: {}", task.payload),
    };

    println!("Enviando resultado...");

    // POST /result
    let response = client
        .post("http://localhost:3000/result")
        .json(&result)
        .send()
        .await?
        .text()
        .await?;

    println!("Respuesta del servidor: {}", response);

    Ok(())
}

