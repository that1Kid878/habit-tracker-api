use axum::extract::{Path, Query, State};
use sqlx::types::Json;

use crate::{
    dto::{
        CreateHabitLogRequest, CreateHabitRequest, EditHabitRequest, GetHabitLogQuery,
        GetHabitQuery,
    },
    models::{Habit, HabitLog},
    repos::{HabitRepo, habit_log::HabitLogRepo},
    responses::{AppError, AppResult},
};

pub struct AppState {
    pub habitRepo: HabitRepo,
    pub habitLogRepo: HabitLogRepo,
}

pub async fn create_habit(
    State(state): State<AppState>,
    Json(payload): Json<CreateHabitRequest>,
) -> Result<AppResult<Habit>, AppError> {
    let result = state.habitRepo.create(payload).await?;
    Ok(AppResult::Created(result))
}

pub async fn get_habit(
    State(state): State<AppState>,
    Query(payload): Query<GetHabitQuery>,
) -> Result<AppResult<Vec<Habit>>, AppError> {
    let result = state.habitRepo.get(payload).await?;
    Ok(AppResult::Ok(result))
}

pub async fn edit_habit(
    State(state): State<AppState>,
    Json(payload): Json<EditHabitRequest>,
) -> Result<AppResult<()>, AppError> {
    state.habitRepo.edit(payload).await?;
    Ok(AppResult::Ok(()))
}

pub async fn delete_habit(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<AppResult<()>, AppError> {
    state.habitRepo.delete(id).await?;
    Ok(AppResult::Ok(()))
}

pub async fn get_habit_log(
    State(state): State<AppState>,
    Query(payload): Query<GetHabitLogQuery>,
) -> Result<AppResult<Vec<HabitLog>>, AppError> {
    let result = state.habitLogRepo.get(payload).await?;
    Ok(AppResult::Ok(result))
}

pub async fn create_habit_log(
    State(state): State<AppState>,
    Json(payload): Json<CreateHabitLogRequest>,
) -> Result<AppResult<HabitLog>, AppError> {
    let result = state.habitLogRepo.create(payload).await?;
    Ok(AppResult::Created(result))
}

pub async fn delete_habit_log(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<AppResult<()>, AppError> {
    state.habitLogRepo.delete(id).await?;
    Ok(AppResult::Ok(()))
}
