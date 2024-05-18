use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

#[derive(Debug)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub city: Option<String>,
    pub description: Option<String>,
    pub headline: Option<String>,
    pub goals: Option<String>,
    pub interests: Option<String>,
    pub language: Option<String>,
    pub age: Option<i16>,
    pub education: Option<String>,
    pub hobby: Option<String>,
    pub music: Option<String>,
    pub sport: Option<String>,
    pub books: Option<String>,
    pub food: Option<String>,
    pub worldview: Option<String>,
}

#[derive(Debug)]
pub struct UserTgData {
    pub tg_username: Option<String>,
    pub tg_user_id: Option<i64>,
}

#[derive(Debug)]
pub struct UserContactData {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub telegram: UserTgData,
    pub instagram: Option<String>,
}

#[derive(Debug)]
pub struct AuthData {
    pub password_hash: Option<String>,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, sqlx::Type)]
pub enum ResoAccountType {
    Standard = 0,
    Bad = 1,
    Banned = 2,
    Premium = 3,
}

#[derive(Debug)]
pub struct BaseAccount {
    pub id: i64,
    pub username: Option<String>,
    pub user_data: UserData,
    pub contact_data: UserContactData,
    pub auth_data: AuthData,
    pub user_type: ResoAccountType,
}

impl FromRow<'_, PgRow> for BaseAccount {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(BaseAccount {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            user_data: UserData {
                first_name: row.try_get("first_name")?,
                last_name: row.try_get("last_name")?,
                city: row.try_get("city")?,
                description: row.try_get("description")?,
                headline: row.try_get("headline")?,
                goals: row.try_get("goals")?,
                interests: row.try_get("interests")?,
                language: row.try_get("language")?,
                age: row.try_get("age")?,
                education: row.try_get("education")?,
                hobby: row.try_get("hobby")?,
                music: row.try_get("music")?,
                sport: row.try_get("sport")?,
                books: row.try_get("books")?,
                food: row.try_get("food")?,
                worldview: row.try_get("worldview")?,
            },
            contact_data: UserContactData {
                email: row.try_get("email")?,
                phone: row.try_get("phone")?,
                telegram: UserTgData {
                    tg_username: row.try_get("tg_username")?,
                    tg_user_id: row.try_get("tg_user_id")?,
                },
                instagram: row.try_get("instagram")?,
            },
            auth_data: AuthData {
                password_hash: row.try_get("password_hash")?,
            },
            user_type: row.try_get("user_type")?,
        })
    }
}
