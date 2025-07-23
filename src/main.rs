use axum::{Router, routing::get, response::Json};
use serde_json::{json, Value};
use crate::db::connect_db;
#[tokio::main]
async fn main() {
    connect_db().await;
    let app = Router::new()
        .route("/", get(root))
        .route("/json", get(json_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json_handler() -> Json<Value> {
    Json(json!({ "data": 42 }))
}