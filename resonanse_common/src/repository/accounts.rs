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
        if let Some(tg_user_id) = account.contact_data.telegram.tg_user_id {
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

    pub async fn get_user_city(&self, tg_user_id: i64) -> Result<Option<String>> {
        let user_city: Option<String> = sqlx::query(
            r#"select city from user_accounts
                where tg_user_id=$1
            "#,
        )
            .bind(tg_user_id)
            .fetch_one(&self.db_pool)
            .await?
            .try_get::<_, usize>(0)?;

        Ok(user_city)
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

    pub async fn create_user(&self, account: BaseAccount) -> Result<BaseAccount, sqlx::error::Error> {
        let created_account: BaseAccount = sqlx::query_as(
            r#"
            INSERT INTO resonanse_users
            (
                username, first_name, last_name, city, description,
                headline, goals, interests, language, age, education,
                hobby, music, sport, books, food, worldview,
                email, phone, tg_username, tg_user_id, instagram,
                password_hash, user_type
            )
            VALUES
            (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16, $17,
                $18, $19, $20, $21, $22,
                $23, $24
            )
            RETURNING *
            "#
        )
            .bind(account.username)
            .bind(account.user_data.first_name)
            .bind(account.user_data.last_name)
            .bind(account.user_data.city)
            .bind(account.user_data.description)
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
            .bind(account.contact_data.email)
            .bind(account.contact_data.phone)
            .bind(account.contact_data.telegram.tg_username)
            .bind(account.contact_data.telegram.tg_user_id)
            .bind(account.contact_data.instagram)
            .bind(account.auth_data.password_hash)
            .bind(account.user_type)
            .fetch_one(&self.db_pool)
            .await?;

        debug!("inserted account: {:?}", created_account);
        Ok(created_account)
    }

    pub async fn set_user_city(&self, user_id: i64, city: String) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE resonanse_users
            SET city = $1
            WHERE id = $2
            "#
        )
            .bind(city)
            .bind(user_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn set_user_description(&self, user_id: i64, description: String) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE resonanse_users
            SET description = $1
            WHERE id = $2
            "#
        )
            .bind(description)
            .bind(user_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
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

    pub async fn count_accounts_with_descriptions(&self) -> Result<i64> {
        debug!("count_account");
        sqlx::query(
            r#"
            select count(*) from user_accounts where description is not null
            "#,
        )
            .fetch_one(&self.db_pool)
            .await?
            .try_get::<_, usize>(0)
    }
}
