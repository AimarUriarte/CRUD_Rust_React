mod usuarios;

use axum::{
    extract::{Path, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use usuarios::Usuario;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);

    // Configurar rutas
    let app = Router::new()
        .route(
            "/usuarios", 
            get(listar_usuarios)
            .post(agregar_usuario))
        .route(
            "/usuarios/:nombre",
            get(buscar_usuario)
                .put(modificar_edad)
                .delete(eliminar_usuario),
        ).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor iniciado en http://{}", addr);

    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---

async fn listar_usuarios() -> Result<Json<Vec<Usuario>>, StatusCode> {
    Ok(Json(usuarios::listar_usuarios()))
}

async fn agregar_usuario(Json(u): Json<Usuario>) -> Result<Json<Vec<Usuario>>, StatusCode> {
    Ok(Json(usuarios::agregar_usuario(&u.nombre, u.edad)))
}

async fn buscar_usuario(Path(nombre): Path<String>) -> Result<Json<Option<Usuario>>, StatusCode> {
    Ok(Json(usuarios::buscar_usuario(&nombre)))
}

async fn modificar_edad(
    Path(nombre): Path<String>,
    Json(u): Json<Usuario>,
) -> Result<Json<Vec<Usuario>>, StatusCode> {
    Ok(Json(usuarios::modificar_edad(&nombre, u.edad)))
}

async fn eliminar_usuario(Path(nombre): Path<String>) -> Result<Json<Vec<Usuario>>, StatusCode> {
    Ok(Json(usuarios::eliminar_usuario(&nombre)))
}
