use sqlx::{Sqlite, SqlitePool, Transaction, query, query_as};

use crate::{
    dto::{CreateHabitRequest, EditHabitRequest},
    models::Habit,
};

pub struct HabitRepo {
    pool: SqlitePool,
}

impl HabitRepo {
    pub async fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Vec<Habit>, sqlx::Error> {
        let habits = query_as!(Habit, "SELECT * FROM habits WHERE username = ?", username)
            .fetch_all(&self.pool)
            .await?;
        Ok(habits)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Habit, sqlx::Error> {
        let habit = query_as!(Habit, "SELECT * FROM habits WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(habit)
    }

    pub async fn get_by_day(&self, day: i32) -> Result<Vec<Habit>, sqlx::Error> {
        let habits = query_as!(
            Habit,
            r#"
            SELECT habits.* FROM habits
            INNER JOIN habit_days ON habits.id = habit_days.habit_id
            WHERE habit_days.day = ?
            "#,
            day
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(habits)
    }

    pub async fn create(&self, payload: CreateHabitRequest) -> Result<Habit, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let habit = query_as!(
            Habit,
            r#"INSERT INTO habits (username, name, description, priority) VALUES ( ?, ?, ?, ?) RETURNING *"#,
            payload.username,
            payload.name,
            payload.description,
            payload.priority
        ).fetch_one(&mut *tx).await?;

        self.insert_days(habit.id, payload.days, &mut tx).await?;

        tx.commit().await?;
        Ok(habit)
    }

    pub async fn edit(&self, payload: EditHabitRequest) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        query!(
            r#"
            UPDATE habits
            SET name = COALESCE(?, name),
                description = COALESCE(?, description),
                priority = COALESCE(?, priority)
            WHERE id = ?
            "#,
            payload.name,
            payload.description,
            payload.priority,
            payload.id,
        )
        .execute(&mut *tx)
        .await?;

        if payload.days.is_none() {
            self.delete_days(payload.id, &mut tx).await?;
            self.insert_days(payload.id, payload.days.unwrap(), &mut tx)
                .await?;
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        query!(r#"DELETE FROM habits WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn insert_days(
        &self,
        habit_id: i64,
        days: Vec<i64>,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), sqlx::Error> {
        for day in days {
            query!(
                "INSERT INTO habit_days (habit_id, day) VALUES (?, ?)",
                habit_id,
                day
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn delete_days(
        &self,
        habit_id: i64,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), sqlx::Error> {
        query!("DELETE FROM habit_days WHERE habit_id = ?", habit_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
