use sqlx::{QueryBuilder, Sqlite, SqlitePool, Transaction, query, query_as};

use crate::{
    dto::{CreateHabitRequest, EditHabitRequest, GetHabitQuery},
    models::Habit,
};

#[derive(Clone)]
pub struct HabitRepo {
    pool: SqlitePool,
}

impl HabitRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, payload: GetHabitQuery) -> Result<Vec<Habit>, sqlx::Error> {
        let mut builder = QueryBuilder::<Sqlite>::new("SELECT * FROM habits WHERE username = ");
        builder.push_bind(payload.username);

        if let Some(id) = payload.id {
            builder.push("AND id = ");
            builder.push_bind(id.to_string());
        }

        if let Some(name) = payload.name {
            builder.push("AND name LIKE ");
            builder.push_bind(format!("%{}%", name));
        }

        if let Some(priority) = payload.priority {
            builder.push("AND priority = ");
            builder.push_bind(priority.to_string());
        }

        if let Some(day) = payload.day {
            builder.push("AND id IN (SELECT habit_id FROM habit_days WHERE day = ");
            builder.push_bind(day.to_string());
            builder.push(")");
        }

        builder.push("LIMIT ");
        builder.push_bind(payload.limit.to_string());

        let habits = builder.build_query_as().fetch_all(&self.pool).await?;
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
