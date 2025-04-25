use std::env::var;

use axum::{Router, routing::get};
use worktrace::env::{keys::WEBAPP_HOST, load_dotenv};

#[tokio::main]
async fn main() {
    unsafe { load_dotenv() }.ok();

    let app = Router::new().route("/", get(|| async { "it works" }));
    let host = var(WEBAPP_HOST).unwrap_or("127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
