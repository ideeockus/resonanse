pub struct UserData {
    first_name: String,
    last_name: String,

    city: String,
    headline: Option<String>,
    about: String, // todo add markdown

    goals: Option<String>,
    interests: Option<String>,

    language: Option<String>,
    age: Option<i16>,
    education: Option<String>,

    hobby: Option<String>,
    music: Option<String>,
    sport: Option<String>,
    books: Option<String>,
    food: Option<String>,
    worldview: Option<String>,
    alcohol: Option<String>,
}

pub struct UserTgData {
    username: Option<String>,
    id: Option<i64>,
}

pub struct UserContactData {
    email: Option<String>,
    phone: Option<String>,
    telegram: UserTgData,
    instagram: Option<String>,
}

pub struct AuthData {
    password_hash: String,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, sqlx::Type)]
pub enum ResoAccountType {
    Standard = 0,
    Bad = 1, // reduced ?
    Banned = 2,
    Premium = 3,
}

pub struct BaseAccount {
    id: i64,
    username: String,
    user_data: UserData,
    contact_data: UserContactData,
    auth_data: AuthData,
    user_type: ResoAccountType,
}
