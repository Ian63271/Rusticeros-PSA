// =============================
// Importaciones
// =============================

// Cliente HTTP para enviar solicitudes a los workers.
// Usamos la librería Reqwest.
use reqwest::Client;

// Serde permite convertir structs ↔ JSON automáticamente.
use serde::{Serialize, Deserialize};

// Librería para crear y guardar imágenes.
use image::{ImageBuffer, Luma};



// =============================
// Estructura de la solicitud
// =============================

// Esta estructura representa el JSON que el master enviará
// a cada worker.
#[derive(Serialize)]
struct MandelbrotRequest {

    // Coordenada inicial en X del bloque a calcular
    x_start: u32,

    // Coordenada final en X
    x_end: u32,

    // Coordenada inicial en Y
    y_start: u32,

    // Coordenada final en Y
    y_end: u32,

    // Ancho total de la imagen
    width: u32,

    // Alto total de la imagen
    height: u32,

    // Máximo de iteraciones del algoritmo
    max_iter: u32,
}



// =============================
// Estructura de la respuesta
// =============================

// Esta estructura representa el JSON que los workers
// devuelven al master.
#[derive(Deserialize)]
struct MandelbrotResponse {

    // Posición inicial del bloque calculado
    x_start: u32,

    // Posición inicial en Y
    y_start: u32,

    // Vector con los valores calculados para cada píxel
    pixels: Vec<u32>,
}



// =============================
// Función para enviar trabajo
// =============================

// Esta función envía una tarea a un worker mediante HTTP.
// El worker ejecuta el cálculo y devuelve los píxeles.
async fn send_task(

    // Cliente HTTP reutilizable
    client: &Client,

    // Dirección del worker
    worker: &str,

    // Datos de la región que se quiere calcular
    req: MandelbrotRequest

) -> MandelbrotResponse {

    client
        // Endpoint del worker
        .post(format!("{}/compute", worker))

        // Convertimos la request a JSON automáticamente
        .json(&req)

        // Enviamos la solicitud
        .send()
        .await
        .unwrap()

        // Convertimos la respuesta JSON a struct
        .json::<MandelbrotResponse>()
        .await
        .unwrap()
}



// =============================
// Función principal
// =============================

// #[tokio::main] inicia el runtime async de Tokio.
// Permite ejecutar tareas asíncronas.
#[tokio::main]
async fn main() {

    // =============================
    // Lista de workers
    // =============================

    //AQUI SE MODIFICO
    let worker_count: u32 = std::env::var("WORKER_COUNT")
    .unwrap_or("20".to_string())
    .parse()
    .unwrap();

    let service_name = std::env::var("SERVICE_NAME").unwrap_or("mandelbrot-worker-service".to_string());
    let namespace = std::env::var("NAMESPACE").unwrap_or("default".to_string());

    let mut workers = Vec::new();

    // En K3s/K8s, los pods de un StatefulSet o con Headless Service 
    // siguen el patrón: nombre-pod-id.nombre-servicio.namespace.svc.cluster.local
    for i in 0..worker_count {
        workers.push(format!("http://mandelbrot-worker-{}.{}.{}.svc.cluster.local:3000", i, service_name, namespace));
    }
    

    println!("Workers detectados: {:?}", workers);



    // =============================
    // Parámetros de la imagen
    // =============================

    // Resolución final de la imagen
    let width: u32 = 1920;
    let height: u32 = 1080;

    // Número máximo de iteraciones del Mandelbrot
    let max_iter: u32 = 1000;



    // =============================
    // Cliente HTTP
    // =============================

    // Creamos un cliente HTTP reutilizable
    let client = Client::new();



    // =============================
    // División del trabajo
    // =============================

    // Calculamos cuántas columnas le corresponden
    // a cada worker.
    let chunk = width / workers.len() as u32;



    // Vector que guardará las tareas asíncronas
    let mut tasks = Vec::new();



    // =============================
    // Envío de tareas a workers
    // =============================

    for (i, worker) in workers.iter().enumerate() {

        // Creamos la región que este worker calculará
        let req = MandelbrotRequest {

            // Inicio de la región en X
            x_start: i as u32 * chunk,

            // Fin de la región
            x_end: (i as u32 + 1) * chunk,

            // La imagen completa en Y
            y_start: 0,
            y_end: height,

            // Dimensiones globales
            width,
            height,

            // Iteraciones
            max_iter,
        };

        // Clonamos datos para moverlos al hilo async
        let worker = worker.to_string();
        let client = client.clone();



        // Lanzamos la tarea asíncrona
        tasks.push(tokio::spawn(async move {

            // Enviamos la tarea al worker
            send_task(&client, &worker, req).await
        }));
    }



    // =============================
    // Recibir resultados
    // =============================

    // Vector donde guardaremos las respuestas
    let mut responses = Vec::new();

    for t in tasks {

        // Esperamos a que cada worker termine
        responses.push(t.await.unwrap());
    }

    println!("Todos los workers terminaron");



    // =============================
    // Crear imagen final
    // =============================

    // Creamos un buffer de imagen en escala de grises.
    let mut img = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width, height);



    // =============================
    // Reconstrucción de la imagen
    // =============================

    for res in responses {

        // Índice dentro del vector de píxeles
        let mut idx = 0;

        // Calculamos el ancho del bloque que devolvió el worker
        let block_width = res.pixels.len() as u32 / height;



        // Recorremos el bloque recibido
        for x in res.x_start..res.x_start + block_width {
            for y in 0..height {

                // Valor de iteración del Mandelbrot
                let val = res.pixels[idx];

                idx += 1;

                // Convertimos iteraciones a color
                let color = (val % 256) as u8;

                // Guardamos el pixel en la imagen final
                img.put_pixel(x, y, Luma([color]));
            }
        }
    }



    // =============================
    // Guardar imagen
    // =============================

    img.save("mandelbrot.png").unwrap();

    println!("Imagen guardada como mandelbrot.png");
}
