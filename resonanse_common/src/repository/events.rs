use chrono::NaiveDateTime;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;
use crate::configuration::POSTGRES_DB_URL;
use crate::models::{BaseEvent, EventSubject, EventType, Location};

pub struct CreateBaseEvent {
    pub is_private: bool,
    pub is_commercial: bool,
    pub title: String,
    pub description: String,
    pub subject: EventSubject,
    pub datetime: NaiveDateTime,
    pub timezone: chrono_tz::Tz,
    pub location: Location,
    pub creator_id: u64,
    pub event_type: EventType,
    pub picture: Uuid,
}

struct EventsRepository {
    db_pool: PgPool,
}

impl EventsRepository {
    pub async fn new() -> Self {
        let conn_url = std::env::var(POSTGRES_DB_URL).unwrap();
        let pool = sqlx::PgPool::connect(&conn_url).await.unwrap();

        Self {
            db_pool: pool,
        }
    }
    pub async fn create_event(&self, event: CreateBaseEvent) -> BaseEvent {
        query_as!(BaseEvent,
            r#"insert into resonanse_events
            (
            is_private, is_commercial, title, description, subject,
            datetime, timezone, location, creator_id, event_type, picture
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            returning id
            "#,
            event.is_private,
            event.is_commercial,
            event.title,
            event.description,
            event.subject,
            event.datetime,
            event.timezone,
            event.location,
            event.creator_id,
            event.event_type,
            event.picture,
        ).fetch_all(&self.db_pool)
            .await
    }

    pub fn get_events() -> Vec<BaseEvent> {
        todo!()
        // query_as!(BaseEvent,
        //     r#"select
        //     from resonanse_events
        //     where
        //     order by id
        //     offset {} rows
        //     fetch next {} rows only
        //     "#
        //
        // )
    }

    pub fn delete_events() {}

    pub fn create_event_tg_binding() {}

    pub fn create_tg_event_link() {}
}
