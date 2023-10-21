pub struct UserData {
    // username: String,
    first_name: String,
    last_name: String,

    city: String,
    headline: String,
    about: String, // todo add markdown

    goals: String,
    interests: String,

    language: Option<String>,
    age: Option<u32>,
    education: Option<String>,

    hobby: Option<String>,
    music: Option<String>,
    sport: Option<String>,
    books: Option<String>,
    food: Option<String>,
    worldview: Option<String>,
    alcohol: Option<String>,
}

pub struct UserContactData {
    email: Option<String>,
    phone: Option<String>,
    telegram: Option<String>,
    instagram: Option<String>,
}

pub struct AuthData {
    password_hash: String,
}

pub struct BaseAccount {
    id: u64,
    username: String,
    user_data: UserData,
    contact_data: UserContactData,
    auth_data: AuthData,
}
