use chrono::NaiveDateTime;
use sqlx::{PgPool, query, query_as, Result};
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
    // pub timezone: chrono_tz::Tz,
    pub location: Location,
    pub creator_id: i64,
    pub event_type: EventType,
    pub picture: Uuid,
}

pub struct EventsRepository {
    db_pool: PgPool,
}

impl EventsRepository {
    // todo call new in OnceCell
    pub async fn new() -> Self {
        let conn_url = std::env::var(POSTGRES_DB_URL).unwrap();
        let pool = sqlx::PgPool::connect(&conn_url).await.unwrap();

        Self {
            db_pool: pool,
        }
    }
    pub async fn create_event(&self, event: CreateBaseEvent) -> Result<BaseEvent> {
        // query_as!(BaseEvent,
        //     r#"insert into resonanse_events
        //     (
        //     is_private, is_commercial, title, description, subject,
        //     datetime, location_latitude, location_longitude,
        //     location_title, creator_id, event_type, picture
        //     )
        //     values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        //     returning event_type as "event_type: EventType", subject as "subject: EventSubject",
        //     is_private, is_commercial, title, description,
        //     datetime, location_latitude, location_longitude,
        //     location_title, creator_id, picture
        //     "#,
        //     event.is_private,
        //     event.is_commercial,
        //     event.title,
        //     event.description,
        //     event.subject as i32,
        //     event.datetime,
        //     // event.timezone,
        //     event.location.latitude,
        //     event.location.longitude,
        //     event.location.title,
        //     event.creator_id,
        //     event.event_type as i32,
        //     event.picture,
        // ).fetch_one(&self.db_pool)
        //     .await
        let created_event = query!(
            r#"insert into resonanse_events
            (
            id, is_private, is_commercial, title, description, subject,
            datetime, location_latitude, location_longitude,
            location_title, creator_id, event_type, picture
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            returning is_private, is_commercial, title, description,
            datetime, location_latitude, location_longitude,
            location_title, creator_id, picture, id, creation_time,
            event_type as "event_type: EventType", subject as "subject: EventSubject"
            "#,
            Uuid::new_v4(),
            event.is_private,
            event.is_commercial,
            event.title,
            event.description,
            event.subject as i32,
            event.datetime,
            // event.timezone,
            event.location.latitude,
            event.location.longitude,
            event.location.title,
            event.creator_id,
            event.event_type as i32,
            event.picture,
        ).fetch_one(&self.db_pool)
            .await?;

        Ok(BaseEvent {
            id: created_event.id,
            is_private: created_event.is_private,
            is_commercial: created_event.is_commercial,
            title: created_event.title,
            description: created_event.description,
            subject: created_event.subject,
            datetime: created_event.datetime,
            location: Location {
                latitude: created_event.location_latitude,
                longitude: created_event.location_longitude,
                title: created_event.location_title,
            },
            creator_id: created_event.creator_id,
            event_type: created_event.event_type,
            picture: created_event.picture,
            creation_time: created_event.creation_time,
        })
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
