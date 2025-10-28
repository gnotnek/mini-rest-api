mod db;
mod error;
mod modules;

use axum::Router;
use modules::notes;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let app = Router::new()
        .nest("/notes", notes::routes())
        .with_state(pool);

    let addr = "127.0.0.1:8080";
    println!("🚀 Server running at http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
