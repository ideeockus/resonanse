use std::collections::HashMap;

use log::debug;
use sqlx::{PgPool, Result, Row};
use uuid::Uuid;

use crate::EventSubjectFilter;
use crate::models::{BaseEvent, EventSubject};

#[derive(Debug)]
pub struct EventsRepository {
    db_pool: PgPool,
}

impl EventsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }

    pub async fn create_event(&self, event: BaseEvent) -> Result<BaseEvent, sqlx::error::Error> {
        let created_event: BaseEvent = sqlx::query_as(
            r#"
            INSERT INTO resonanse_events
            (
                id, title, description, datetime_from, datetime_to, city, venue_title, venue_address,
                venue_lat, venue_lon, image_url, local_image_path, price_price, price_currency, tags,
                contact, service_id, service_type, service_data
            )
            VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            RETURNING *
            "#
        )
            .bind(Uuid::new_v4())
            .bind(&event.title)
            .bind(&event.description)
            .bind(event.datetime_from)
            .bind(&event.datetime_to)
            .bind(&event.city)
            .bind(&event.venue.title)
            .bind(&event.venue.address)
            .bind(&event.venue.longitude)
            .bind(&event.venue.latitude)
            .bind(&event.image_url)
            .bind(&event.local_image_path)
            .bind(&event.price_price)
            .bind(&event.price_currency)
            .bind(&event.tags)
            .bind(&event.contact)
            .bind(&event.service_id)
            .bind(&event.service_type)
            .bind(&event.service_data)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(created_event)
    }

    pub async fn get_all_events(&self) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where datetime_from >= current_date
            order by datetime_from
            "#,
        )
            .fetch_all(&self.db_pool)
            .await;

        events
    }

    pub async fn get_events_by_title_substr(&self, title: &str) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where title like $1
            "#,
        )
            .bind(format!("%{}%", title))
            .fetch_all(&self.db_pool)
            .await;

        events
    }

    pub async fn get_all_public_events(&self, page: i64, page_size: i64) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where is_private=false and datetime_from >= current_date
            order by datetime_from
            offset $1 rows
            fetch next $2 rows only
            "#,
        )
            .bind(page * page_size)
            .bind(page_size)
            .fetch_all(&self.db_pool)
            .await;

        events
    }

    pub async fn get_public_events(
        &self,
        page: i64,
        page_size: i64,
        events_subject_filter: &EventSubjectFilter,
    ) -> Result<Vec<BaseEvent>> {
        let filters_vec = events_subject_filter
            .0
            .iter()
            .filter(|(_, f)| **f)
            .map(|(f, _)| (*f as i32))
            .collect::<Vec<_>>();

        if filters_vec.is_empty() {
            return Ok(Vec::new());
        }

        let filter_params_len = filters_vec.len();
        let filter_params = (1..=filter_params_len)
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");
        let query_str = format!(
            r#"select *
            from resonanse_events
            WHERE subject IN ( { } ) and is_private=false and datetime_from >= current_date
            order by datetime_from
            offset ${} rows
            fetch next ${} rows only
            "#,
            filter_params,
            filter_params_len + 1,
            filter_params_len + 2,
        );
        debug!("get_public_events builded query: {}", query_str);

        let mut events_query = sqlx::query_as(&query_str);
        for subj_i32 in filters_vec {
            events_query = events_query.bind(subj_i32);
        }

        let events: Result<Vec<BaseEvent>> = events_query
            .bind(page * page_size)
            .bind(page_size)
            .fetch_all(&self.db_pool)
            .await;
        events
    }

    pub async fn get_event_by_uuid(&self, uuid: Uuid) -> Result<BaseEvent> {
        let event: Result<BaseEvent> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where id=$1
            "#,
        )
            .bind(uuid)
            .fetch_one(&self.db_pool)
            .await;

        event
    }

    pub async fn delete_event(&self, event_uuid: Uuid, _deleted_by_id: i64) -> Result<(), sqlx::error::Error> {
        let deleting_event = self.get_event_by_uuid(event_uuid).await?;

        let _deleted_event: BaseEvent = sqlx::query_as(
            r#"
            INSERT INTO deleted_events
            (
                id, title, description, datetime_from, datetime_to, city, venue_title, venue_address,
                venue_lat, venue_lon, image_url, local_image_path, price_price, price_currency, tags,
                contact, service_id, service_type, service_data
            )
            VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            RETURNING *
            "#
        )
            .bind(deleting_event.id)
            .bind(&deleting_event.title)
            .bind(&deleting_event.description)
            .bind(deleting_event.datetime_from)
            .bind(&deleting_event.datetime_to)
            .bind(&deleting_event.city)
            .bind(&deleting_event.venue.title)
            .bind(&deleting_event.venue.address)
            .bind(&deleting_event.venue.latitude)
            .bind(&deleting_event.venue.longitude)
            .bind(&deleting_event.image_url)
            .bind(&deleting_event.local_image_path)
            .bind(&deleting_event.price_price)
            .bind(&deleting_event.price_currency)
            .bind(&deleting_event.tags)
            .bind(&deleting_event.contact)
            .bind(&deleting_event.service_id)
            .bind(&deleting_event.service_type)
            .bind(&deleting_event.service_data)
            .fetch_one(&self.db_pool)
            .await?;

        let result = sqlx::query(
            r#"
            DELETE FROM resonanse_events
            WHERE id = $1
            "#
        )
            .bind(event_uuid)
            .execute(&self.db_pool)
            .await?;

        debug!("delete_events result {:?}", result);
        Ok(())
    }

    pub async fn create_event_tg_binding(&self, post_id: i64, event_id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"insert into event_tg_table
            (post_id, creator_id)
            values ($1, $2)
            "#,
        )
            .bind(post_id)
            .bind(event_id)
            .execute(&self.db_pool)
            .await?;
        debug!("event_tg_table result {:?}", result);

        Ok(())
    }

    pub async fn count_events(&self) -> Result<i64> {
        sqlx::query(
            r#"select count(*)
            from resonanse_events
            "#,
        )
            .fetch_one(&self.db_pool)
            .await?
            .try_get::<_, usize>(0)
    }
}
