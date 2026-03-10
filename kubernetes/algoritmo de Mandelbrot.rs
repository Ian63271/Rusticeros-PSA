// =============================
// Importaciones necesarias
// =============================

// Importamos componentes del framework Axum.
// Axum es un framework web moderno para Rust que funciona sobre Tokio.
use axum::{
    routing::post,   // Permite definir rutas HTTP tipo POST
    Router,          // Estructura principal para construir el servidor y definir rutas
    Json,            // Extractor y respuesta automática para trabajar con JSON
};

// Serde es la librería estándar en Rust para serializar y deserializar datos.
// La usamos para convertir automáticamente entre JSON y structs.
use serde::{Deserialize, Serialize};

// Tipo que representa una dirección IP + puerto.
// Lo usaremos para definir dónde escuchará el servidor.
use std::net::SocketAddr;



// =============================
// Estructura de la solicitud (Request)
// =============================

// Esta estructura representa el JSON que el worker recibirá.
// #[derive(Deserialize)] permite convertir automáticamente
// el JSON entrante en esta estructura.
#[derive(Deserialize)]
struct MandelbrotRequest {

    // Coordenada inicial en el eje X del bloque a calcular
    x_start: u32,

    // Coordenada final en el eje X (no inclusiva)
    x_end: u32,

    // Coordenada inicial en el eje Y
    y_start: u32,

    // Coordenada final en el eje Y (no inclusiva)
    y_end: u32,

    // Ancho total de la imagen global.
    // Se usa para mapear coordenadas de píxel al plano complejo.
    width: u32,

    // Alto total de la imagen global.
    height: u32,

    // Número máximo de iteraciones del algoritmo de Mandelbrot.
    // Más iteraciones = mayor precisión pero más uso de CPU.
    max_iter: u32,
}



// =============================
// Estructura de la respuesta (Response)
// =============================

// #[derive(Serialize)] permite convertir esta estructura
// automáticamente en JSON para enviarla como respuesta HTTP.
#[derive(Serialize)]
struct MandelbrotResponse {

    // Coordenada inicial X del bloque calculado.
    // Se incluye para que el nodo master sepa dónde ubicar
    // estos píxeles en la imagen final.
    x_start: u32,

    // Coordenada inicial Y del bloque.
    y_start: u32,

    // Vector que contiene los valores calculados para cada píxel.
    // Cada valor representa cuántas iteraciones tomó el punto
    // antes de escapar del conjunto de Mandelbrot.
    pixels: Vec<u32>,
}



// =============================
// Función que calcula Mandelbrot para un punto
// =============================

// Esta función implementa el algoritmo matemático
// del conjunto de Mandelbrot:
//
//    z = z^2 + c
//
// donde z inicia en 0 y c es el punto complejo evaluado.
fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {

    // Parte real de z
    let mut z_re = 0.0;

    // Parte imaginaria de z
    let mut z_im = 0.0;

    // Contador de iteraciones
    let mut i = 0;

    // Mientras:
    // 1) El módulo de z sea menor o igual a 2 (|z|² <= 4)
    // 2) No se alcance el máximo de iteraciones
    while z_re * z_re + z_im * z_im <= 4.0 && i < max_iter {

        // Aplicamos la fórmula:
        // (a + bi)^2 = a^2 - b^2 + 2abi
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;

        // Actualizamos z
        z_re = new_re;
        z_im = new_im;

        // Incrementamos contador
        i += 1;
    }

    // Retornamos el número de iteraciones realizadas.
    // Si alcanza max_iter, el punto probablemente pertenece al conjunto.
    i
}



// =============================
// Handler HTTP del Worker
// =============================

// Esta función es el endpoint que responde a:
// POST /compute
//
// Axum automáticamente:
// - Extrae el JSON del body
// - Lo convierte en MandelbrotRequest
// - Devuelve JSON como respuesta
async fn compute(Json(req): Json<MandelbrotRequest>) -> Json<MandelbrotResponse> {

    // Vector donde almacenaremos los resultados de cada píxel.
    let mut pixels = Vec::new();

    // Recorremos únicamente el bloque asignado al worker.
    // Esto permite que múltiples workers procesen diferentes
    // partes de la imagen en paralelo.
    for x in req.x_start..req.x_end {
        for y in req.y_start..req.y_end {

            // Convertimos coordenadas de píxel (x,y)
            // a coordenadas en el plano complejo.
            //
            // Rango típico:
            // Real: [-2.5, 1]
            // Imaginario: [-1, 1]
            let c_re = (x as f64 / req.width as f64) * 3.5 - 2.5;
            let c_im = (y as f64 / req.height as f64) * 2.0 - 1.0;

            // Calculamos Mandelbrot para ese punto
            let i = mandelbrot(c_re, c_im, req.max_iter);

            // Guardamos el resultado en el vector
            pixels.push(i);
        }
    }

    // Retornamos los resultados en formato JSON.
    Json(MandelbrotResponse {
        x_start: req.x_start,
        y_start: req.y_start,
        pixels,
    })
}



// =============================
// Función principal
// =============================

// #[tokio::main] inicializa automáticamente el runtime async de Tokio.
// Tokio es el motor que permite programación asíncrona en Rust.
#[tokio::main]
async fn main() {

    // Creamos el router del servidor.
    // Definimos una única ruta:
    // POST /compute
    let app = Router::new().route("/compute", post(compute));

    // Definimos la dirección donde el worker escuchará.
    //
    // 0.0.0.0 significa:
    // "Escuchar en todas las interfaces de red disponibles".
    //
    // Esto es importante para Docker y sistemas distribuidos,
    // porque permite acceso desde otras máquinas.
    //
    // Puerto: 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Mensaje en consola indicando que el worker está activo.
    println!("Worker listening on {}", addr);

    // Creamos el servidor HTTP:
    // 1) Bind al puerto
    // 2) Asociamos el router
    // 3) Ejecutamos indefinidamente
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
