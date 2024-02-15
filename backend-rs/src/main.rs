use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let addr = String::from("0.0.0.0:5001");
    println!("Starting HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
