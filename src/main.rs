use axum::Router;

use crate::router::todo_router::{create_todo_router, todo_seed};

mod config;
mod handlers;
mod models;
mod router;
mod schema;

#[tokio::main]
async fn main() {
    // run seeder
    todo_seed().await;

    let todo_router = create_todo_router();

    let app = Router::new().nest("/todos", todo_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running server on http://localhost:3000");
    let serve = axum::serve(listener, app).await;
    match serve {
        Err(err) => {
            println!("Error while running server\nError: {}", err.to_string())
        }
        Ok(_) => {}
    }
}
