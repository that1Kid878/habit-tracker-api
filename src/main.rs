mod dto;
mod errors;
mod handler;
mod models;
mod repos;
mod responses;

use std::env;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;

use crate::{
    handler::{
        AppState, create_habit, create_habit_log, delete_habit, delete_habit_log, edit_habit,
        get_habit, get_habit_log,
    },
    repos::{HabitRepo, habit_log::HabitLogRepo},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DA TABASE_URL").unwrap_or_else(|_| {
        println!("Unable to use .env's DATABASE_URL, using memory...");
        "sqlite::memory:".to_string()
    });
    println!("Database URL recieved");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Pool connected");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations successful");

    let habit_repo = HabitRepo::new(pool.clone());
    let habit_log_repo = HabitLogRepo::new(pool.clone());
    let state = AppState {
        habit_repo: habit_repo,
        habit_log_repo: habit_log_repo,
    };

    let app = Router::new()
        .route("/habits/new", post(create_habit))
        .route("/habits", get(get_habit))
        .route("/habits", put(edit_habit))
        .route("/habits/{id}", delete(delete_habit))
        .route("/log", post(create_habit_log))
        .route("/log", get(get_habit_log))
        .route("/log/{id}", delete(delete_habit_log))
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("App has been set up");

    axum::serve(listener, app).await?;
    Ok(())
}
