use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Habit {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub last_edited: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct HabitLog {
    pub id: i64,
    pub habit_id: i64,
    pub day: i64,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}
