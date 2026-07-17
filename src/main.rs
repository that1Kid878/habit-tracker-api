mod dto;
mod errors;
mod handler;
mod models;
mod repos;
mod responses;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new();
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("App has been set up");

    axum::serve(listener, app).await?;
    Ok(())
}
