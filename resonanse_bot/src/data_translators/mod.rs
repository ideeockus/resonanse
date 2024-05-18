use log::warn;
use resonanse_common::models::{
    AuthData, BaseAccount, ResoAccountType, UserContactData, UserData, UserTgData,
};
use std::cmp::Ordering;

pub fn fill_base_account_from_teloxide_user(user: &teloxide::types::User) -> BaseAccount {
    let user_data = UserData {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone().unwrap_or("".to_string()),
        city: None,
        description: None,
        headline: None,
        goals: None,
        interests: None,
        language: None,
        age: None,
        education: None,
        hobby: None,
        music: None,
        sport: None,
        books: None,
        food: None,
        worldview: None,
    };

    let tg_user_id = match user.id.0.cmp(&(i64::MAX as u64)) {
        Ordering::Greater => {
            warn!("tg_user_id {:?} is greater than MAX i64", user.id.0);
            None
        }
        _ => Some(user.id.0 as i64),
    };

    let contact_data = UserContactData {
        email: None,
        phone: None,
        telegram: UserTgData {
            tg_username: user.username.clone(),
            tg_user_id,
        },
        instagram: None,
    };

    BaseAccount {
        id: 0, // will be filled on insert to db
        username: user.username.clone(),
        user_data,
        contact_data,
        auth_data: AuthData {
            password_hash: None,
        },
        user_type: ResoAccountType::Standard,
    }
}
