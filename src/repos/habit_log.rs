use chrono::NaiveDate;
use sqlx::{SqlitePool, query, query_as};

use crate::{
    dto::{CreateHabitLogRequest, GetHabitLogByScopeRequest},
    models::HabitLog,
};

pub struct HabitLogRepo {
    pool: SqlitePool,
}

impl HabitLogRepo {
    pub async fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i64) -> Result<HabitLog, sqlx::Error> {
        let log = query_as!(HabitLog, "SELECT * FROM habits_log WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(log)
    }

    pub async fn get_by_date(&self, id: i64, date: NaiveDate) -> Result<HabitLog, sqlx::Error> {
        let log = query_as!(
            HabitLog,
            "SELECT * FROM habits_log WHERE id = ? AND DATE(created_at) = DATE(?)",
            id,
            date
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(log)
    }

    pub async fn get_by_scope(
        &self,
        payload: GetHabitLogByScopeRequest,
    ) -> Result<Vec<HabitLog>, sqlx::Error> {
        let logs = query_as!(
            HabitLog,
            r#"SELECT * FROM habits_log WHERE id = ? AND DATE(created_at) BETWEEN DATE(?) AND DATE(?)"#,
            payload.habit_id,
            payload.to,
            payload.from)
        .fetch_all(&self.pool)
        .await?;
        Ok(logs)
    }

    pub async fn create(&self, payload: CreateHabitLogRequest) -> Result<HabitLog, sqlx::Error> {
        let log = query_as!(
            HabitLog,
            r#"INSERT INTO habits_log (habit_id, completed) VALUES ( ?, ?) RETURNING *"#,
            payload.habit_id,
            payload.completed
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(log)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        query!(r#"DELETE FROM habits WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
