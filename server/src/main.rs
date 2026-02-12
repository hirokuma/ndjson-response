use axum::{
    Json, Router,
    body::Body,
    response::{IntoResponse, Response},
    routing::get,
};
use async_stream::stream;
use serde::Serialize;
use futures_core::Stream;

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
    let body_stream = get_item_stream(10);
    Response::builder()
        .header("content-type", "application/x-ndjson")
        .body(Body::from_stream(body_stream))
        .unwrap()
}

fn get_item_stream(num: i32) -> impl Stream<Item = std::io::Result<String>> {
    stream! {
        for i in 0..num {
            let item = Item {
                id: i,
                data: format!("data-{}", i),
            };
            let mut json = serde_json::to_string(&item).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            json.push('\n');
            yield Ok::<_, std::io::Error>(json);

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
