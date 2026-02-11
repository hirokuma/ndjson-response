use axum::{
    Json, Router,
    body::Body,
    response::{IntoResponse, Response},
    routing::get,
};
use futures::stream::{self, StreamExt};
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/json", get(handler_json))
        .route("/stream", get(handler_stream));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await
}

#[derive(Serialize)]
struct Item {
    id: i32,
    data: String,
}

async fn handler_json() -> Json<Item> {
    let item = Item {
        id: 12345,
        data: "data-0000".to_owned(),
    };
    Json(item)
}

async fn handler_stream() -> impl IntoResponse {
    let item_stream = stream::iter(0..10).map(|i| {
        let item = Item {
            id: i,
            data: format!("data-{}", i),
        };
        let mut json = serde_json::to_string(&item).unwrap();
        json.push('\n');
        Ok::<_, std::io::Error>(json)
    });
    Response::builder()
        .header("content-type", "application/x-ndjson")
        .body(Body::from_stream(item_stream))
        .unwrap()
}
