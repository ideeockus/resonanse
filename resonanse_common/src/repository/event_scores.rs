use sqlx::{PgPool, Result};
use uuid::Uuid;
use crate::models::{EventScore, EventScoreType};

#[derive(Debug)]
pub struct EventScoresRepository {
    db_pool: PgPool,
}

impl EventScoresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }
    pub async fn set_event_score_by_user(&self, event_id: Uuid, user_id: i64, score: EventScoreType) -> Result<EventScore> {
        let event_score: EventScore = sqlx::query_as(
            r#"insert into user_likes
            (
            event_id, user_id, event_score
            )
            values ($1, $2, $3)
            on conflict (user_id, event_id) do update
            set event_score = excluded.event_score
            returning *
            "#,
        )
            .bind(event_id)
            .bind(user_id)
            .bind(score as i64)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(event_score)
    }

    pub async fn get_event_scores_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<EventScore>, sqlx::Error> {
        let event_scores: Vec<EventScore> = sqlx::query_as(
            r#"
                SELECT user_id, event_id, event_score
                FROM user_likes
                WHERE user_id = $1
                "#,
        )
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(event_scores)
    }
}