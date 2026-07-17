use sqlx::{QueryBuilder, Sqlite, SqlitePool, query, query_as};

use crate::{
    dto::{CreateHabitLogRequest, GetHabitLogQuery},
    models::HabitLog,
};

#[derive(Clone)]
pub struct HabitLogRepo {
    pool: SqlitePool,
}

impl HabitLogRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, payload: GetHabitLogQuery) -> Result<Vec<HabitLog>, sqlx::Error> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            "SELECT * FROM habits_log WHERE habit_id IN (SELECT id FROM habits WHERE username = ",
        );
        builder.push_bind(payload.username);
        builder.push(")");

        if let Some(id) = payload.id {
            builder.push("AND id = ");
            builder.push_bind(id.to_string());
        }

        if let Some(habit_id) = payload.habit_id {
            builder.push("AND habit_id = ");
            builder.push_bind(habit_id.to_string());
        }

        if let Some(to) = payload.to
            && let Some(from) = payload.from
        {
            builder.push("AND DATE(created_at) BETWEEN DATE(");
            builder.push_bind(to.to_string());
            builder.push(") AND DATE(");
            builder.push_bind(from.to_string());
            builder.push(")");
        }

        builder.push("LIMIT ");
        builder.push_bind(payload.limit.to_string());

        let habits = builder.build_query_as().fetch_all(&self.pool).await?;
        Ok(habits)
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
