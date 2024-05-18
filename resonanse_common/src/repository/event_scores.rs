use std::fmt::{Debug, Formatter};

use chrono::{NaiveDateTime, NaiveTime};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{EventScore, EventScoreType};

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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // let event_score: EventScore = sqlx::query_as(
        //     r#"insert into user_likes
        //     (
        //     event_id, user_id, event_score
        //     )
        //     values ($1, $2, $3)
        //     on conflict (user_id, event_id) do update
        //     set event_score = excluded.event_score
        //     returning *
        //     "#,
        // )
        // .bind(event_id)
        // .bind(user_id)
        // .bind(score.to_string())
        // .fetch_one(&self.db_pool)
        // .await?;

        let now_timestamp = chrono::offset::Local::now().naive_local();
        let query = "INSERT INTO users_interactions (user_id, event_id, interaction_type, interaction_dt) VALUES (?, ?, ?, ?)";
        self.clickhouse_client
            .query(query)
            .bind(user_id)
            .bind(event_id.to_string())
            .bind(score.to_string())
            .bind(now_timestamp.format("%Y-%m-%d %H:%M:%S").to_string())
            .execute()
            .await?;

        Ok(())
    }

    pub async fn get_event_scores_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<EventScore>, Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now_timestamp = chrono::offset::Local::now().naive_local();

        let query = "INSERT INTO users_interactions (user_id, event_id, interaction_type, interaction_dt) VALUES (?, ?, ?, ?)";
        self.clickhouse_client
            .query(query)
            .bind(user_id)
            .bind(event_id.to_string())
            .bind("click")
            .bind(now_timestamp.format("%Y-%m-%d %H:%M:%S").to_string())
            .execute()
            .await?;

        Ok(())
    }

    async fn count_events_by_type(
        &self,
        event_type: &str,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let today = chrono::offset::Local::now().naive_utc().date();
        let today_timestamp = NaiveDateTime::new(today, NaiveTime::default()).timestamp();

        let result: usize = self.clickhouse_client
            .query("SELECT count(*) FROM users_interactions WHERE interaction_type = ? AND toDate(interaction_dt) >= toDate(?)")
            .bind(event_type)
            .bind(today_timestamp)
            .fetch_one().await?;
        Ok(result)
    }

    pub async fn count_clicks_for_today(
        &self,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        self.count_events_by_type("click").await
    }

    pub async fn count_likes_for_today(
        &self,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        self.count_events_by_type("like").await
    }

    pub async fn count_dislikes_for_today(
        &self,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        self.count_events_by_type("dislike").await
    }

    pub async fn count_recommendations_for_today(&self) -> Result<usize, clickhouse::error::Error> {
        let today = chrono::offset::Local::now().naive_utc().date();
        let today_timestamp = NaiveDateTime::new(today, NaiveTime::default()).timestamp();

        let result: usize = self.clickhouse_client
            .query("SELECT count(*) FROM given_recommendations WHERE toDate(recommendation_dt) = toDate(?)")
            .bind(today_timestamp)
            .fetch_one().await?;
        Ok(result)
    }
}
