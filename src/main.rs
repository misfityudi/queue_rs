use axum::{routing::get, Router};
use std::net::SocketAddr;
use crate::db::Db;
use tokio::net::TcpListener;

mod models;
mod db;
mod handlers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = db::Db::new().await.unwrap();

    // Run migrations on startup
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .nest("/blogs", handlers::blogs::routes())
              .nest("/users", handlers::users::routes())
              .nest("/profiles", handlers::profiles::routes())
              .nest("/sessions", handlers::sessions::routes())
        .with_state(db.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "4x4 API is live"
}
