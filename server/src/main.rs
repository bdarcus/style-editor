use axum::{
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/version", get(version));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn version() -> Json<Value> {
    // In the future, we can pull this from csln_core
    Json(json!({
        "service": "style-editor-server",
        "csln_core_version": "0.1.0" 
    }))
}
