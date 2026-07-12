use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "INTEGER")]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Habit {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub repeat_days: i32,
    pub last_edited: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct HabitLog {
    pub id: i32,
    pub habit_id: i32,
    pub day: i32,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
