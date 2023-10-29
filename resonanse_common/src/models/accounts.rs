use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

#[derive(Debug)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,

    pub city: String,
    pub headline: Option<String>,
    pub about: String, // todo add markdown

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
    pub alcohol: Option<String>,
}

#[derive(Debug)]
pub struct UserTgData {
    pub username: Option<String>,
    pub user_id: Option<i64>,
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
    Bad = 1, // reduced ?
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
        Ok(
            BaseAccount {
                id: row.try_get::<_, usize>(0)?,
                username: row.try_get::<_, usize>(1)?,
                user_data: UserData {
                    first_name: row.try_get::<_, usize>(2)?,
                    last_name: row.try_get::<_, usize>(3)?,
                    city: row.try_get::<_, usize>(4)?,
                    headline: row.try_get::<_, usize>(5)?,
                    about: row.try_get::<_, usize>(6)?,
                    goals: row.try_get::<_, usize>(7)?,
                    interests: row.try_get::<_, usize>(8)?,
                    language: row.try_get::<_, usize>(9)?,
                    age: row.try_get::<_, usize>(10)?,
                    education: row.try_get::<_, usize>(11)?,
                    hobby: row.try_get::<_, usize>(12)?,
                    music: row.try_get::<_, usize>(13)?,
                    sport: row.try_get::<_, usize>(14)?,
                    books: row.try_get::<_, usize>(15)?,
                    food: row.try_get::<_, usize>(16)?,
                    worldview: row.try_get::<_, usize>(17)?,
                    alcohol: row.try_get::<_, usize>(18)?,
                },
                contact_data: UserContactData {
                    email: row.try_get::<_, usize>(19)?,
                    phone: row.try_get::<_, usize>(20)?,
                    telegram: UserTgData {
                        username: row.try_get::<_, usize>(21)?,
                        user_id: row.try_get::<_, usize>(22)?
                    },
                    instagram: row.try_get::<_, usize>(23)?,
                },
                auth_data: AuthData {
                    password_hash: row.try_get::<_, usize>(24)?
                },
                user_type: row.try_get::<_, usize>(25)?,
            }
        )
    }
}
