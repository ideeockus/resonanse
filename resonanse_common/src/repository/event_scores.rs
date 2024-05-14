use std::fmt::{Debug, Formatter};
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::{EventScore, EventScoreType};
use crate::UserInteraction;

pub struct EventInteractionRepository {
    db_pool: PgPool,
    clickhouse_client: clickhouse::Client,
}

impl Debug for EventInteractionRepository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventScoresRepository")
            .field("db_pool", &self.db_pool)
            .finish()
    }
}

impl EventInteractionRepository {
    pub fn new(pool: PgPool, clickhouse_client: clickhouse::Client) -> Self {
        Self {
            db_pool: pool,
            clickhouse_client,
        }
    }
    pub async fn set_event_score_by_user(
        &self,
        event_id: Uuid,
        user_id: i64,
        score: EventScoreType,
    ) -> Result<EventScore> {
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

        let mut insert = self.clickhouse_client.insert("users_interactions")?;
        insert.write(&UserInteraction{
            user_id,
            event_id,
            interaction_type: score.to_string(),
            interaction_dt: chrono::offset::Local::now().naive_local(),
        }).await?;

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

    pub async fn add_event_click(
        &self,
        user_id: i64,
        event_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut insert = self.clickhouse_client.insert("users_interactions")?;
        insert.write(&UserInteraction{
            user_id,
            event_id,
            interaction_type: "click".to_string(),
            interaction_dt: chrono::offset::Local::now().naive_local(),
        }).await?;

        Ok(())
    }
}
