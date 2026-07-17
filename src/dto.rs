use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateHabitRequest {
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub days: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EditHabitRequest {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub days: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateHabitLogRequest {
    pub habit_id: i64,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetHabitLogQuery {
    pub username: String,
    pub id: Option<i64>,
    pub habit_id: Option<i64>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub limit: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetHabitQuery {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub username: String,
    pub day: Option<i64>,
    pub priority: Option<i64>,
    pub limit: i64,
}
