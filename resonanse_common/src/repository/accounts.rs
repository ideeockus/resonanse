use crate::models::BaseAccount;
use log::debug;
use sqlx::{PgPool, Result, Row};

#[derive(Debug)]
pub struct AccountsRepository {
    db_pool: PgPool,
}

impl AccountsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }

    /// Creates new user if there are no user with given tg_user_id
    pub async fn create_user_by_tg_user_id(&self, account: BaseAccount) -> Result<BaseAccount> {
        if let Some(tg_user_id) = account.contact_data.telegram.user_id {
            match self.get_user_by_tg_id(tg_user_id).await {
                Err(sqlx::error::Error::RowNotFound) => {
                    debug!("tg_user_id {} not found", tg_user_id);
                }
                Err(err) => {
                    debug!("get_user_by_tg_id returned err {:?}", err);
                    return Err(err);
                }
                Ok(account) => {
                    debug!("account already exists {:?}", account);
                    return Ok(account);
                }
            }
        }

        self.create_user(account).await
    }

    pub async fn get_user_by_tg_id(&self, tg_user_id: i64) -> Result<BaseAccount> {
        let account: BaseAccount = sqlx::query_as(
            r#"select * from user_accounts
                where tg_user_id=$1
            "#,
        )
        .bind(tg_user_id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(account)
    }

    pub async fn get_account_id_by_tg_user_id(&self, tg_user_id: i64) -> Result<i64> {
        debug!("searching account_id by tg_user_id {}", tg_user_id);
        let account_id: Result<i64> = sqlx::query(
            r#"
            select id from user_accounts
            where tg_user_id=$1
            "#,
        )
        .bind(tg_user_id)
        .fetch_one(&self.db_pool)
        .await?
        .try_get::<_, usize>(0);

        debug!("account_id {:?}", account_id);

        account_id
    }

    pub async fn create_user(&self, account: BaseAccount) -> Result<BaseAccount> {
        let created_account: BaseAccount = sqlx::query_as(
            r#"insert into user_accounts
            (
            username, first_name, last_name, city, about,
            headline, goals, interests, language, age, education,
            hobby, music, sport, books, food, worldview, alcohol,
            email, phone, tg_username, tg_user_id, instagram,
            password_hash,
            user_type
            )
            values (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10, $11,
            $12, $13, $14, $15, $16, $17, $18,
            $19, $20, $21, $22, $23,
            $24,
            $25
            )
            returning *
            "#,
        )
        .bind(account.username)
        .bind(account.user_data.first_name)
        .bind(account.user_data.last_name)
        .bind(account.user_data.city)
        .bind(account.user_data.about)
        .bind(account.user_data.headline)
        .bind(account.user_data.goals)
        .bind(account.user_data.interests)
        .bind(account.user_data.language)
        .bind(account.user_data.age)
        .bind(account.user_data.education)
        .bind(account.user_data.hobby)
        .bind(account.user_data.music)
        .bind(account.user_data.sport)
        .bind(account.user_data.books)
        .bind(account.user_data.food)
        .bind(account.user_data.worldview)
        .bind(account.user_data.alcohol)
        .bind(account.contact_data.email)
        .bind(account.contact_data.phone)
        .bind(account.contact_data.telegram.username)
        .bind(account.contact_data.telegram.user_id)
        .bind(account.contact_data.instagram)
        .bind(account.auth_data.password_hash)
        .bind(account.user_type)
        .fetch_one(&self.db_pool)
        .await?;

        // debug!("inserted account: {:?}", created_account.get::<i64, usize>(0));
        debug!("inserted account: {:?}", created_account);

        // Ok(BaseEvent {
        //     id: created_event.id,
        //     is_private: created_event.is_private,
        //     is_commercial: created_event.is_commercial,
        //     title: created_event.title,
        //     description: created_event.description,
        //     subject: created_event.subject,
        //     datetime: created_event.datetime,
        //     location: Location {
        //         latitude: created_event.location_latitude,
        //         longitude: created_event.location_longitude,
        //         title: created_event.location_title,
        //     },
        //     creator_id: created_event.creator_id,
        //     event_type: created_event.event_type,
        //     picture: created_event.picture,
        //     creation_time: created_event.creation_time,
        // })

        Ok(created_account)
    }

    pub async fn count_accounts(&self) -> Result<i64> {
        debug!("count_account");
        sqlx::query(
            r#"
            select count(*) from user_accounts
            "#,
        )
        .fetch_one(&self.db_pool)
        .await?
        .try_get::<_, usize>(0)
    }
}
