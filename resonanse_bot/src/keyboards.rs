use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup};
use resonanse_common::models::EventSubject;


macro_rules! kb_button {
    ($s:expr) => {
        InlineKeyboardButton::new(
                $s,
                InlineKeyboardButtonKind::CallbackData($s.to_string()),
            )
    };
}

pub fn get_inline_kb_choose_subject() -> ReplyMarkup {
    let buttons = [
        vec![
            kb_button!(EventSubject::Social),
            kb_button!(EventSubject::Acquaintance),
            kb_button!(EventSubject::Charity),
        ],
        vec![
            kb_button!(EventSubject::Culture),
            kb_button!(EventSubject::Business),
            kb_button!(EventSubject::Education),
        ],
        vec![
            kb_button!(EventSubject::Professional),
            kb_button!(EventSubject::Sport),
            kb_button!(EventSubject::Interests),
        ],
        vec![
            kb_button!(EventSubject::Other),
        ],
    ];

    let keyboard = InlineKeyboardMarkup::new(buttons);
    ReplyMarkup::InlineKeyboard(keyboard)
}

pub fn get_inline_kb_view_event(map_link: String) -> ReplyMarkup {
    let map_link_btn = InlineKeyboardButton::new(
        "Место на карте",
        InlineKeyboardButtonKind::Url(map_link.parse().unwrap()),
    );
    let buttons = [[map_link_btn]];

    let keyboard = InlineKeyboardMarkup::new(buttons);

    ReplyMarkup::InlineKeyboard(keyboard)
}


pub const EDIT_PUBLICITY_TRUE_CALLBACK: &str = "EDIT_PUBLICITY_TRUE";
pub const EDIT_PUBLICITY_FALSE_CALLBACK: &str = "EDIT_PUBLICITY_FALSE";
pub const REFILL_EVENT_AGAIN_CALLBACK: &str = "REFILL_EVENT_AGAIN";
pub const CREATE_EVENT_CALLBACK: &str = "CREATE_EVENT_CALLBACK";

pub fn get_inline_kb_edit_new_event(is_public: bool, map_link: Option<String>) -> InlineKeyboardMarkup {
    let edit_publicity_btn = InlineKeyboardButton::new(
        if is_public { "Публичное [✅]" } else { "Публичное [❌]" },
        InlineKeyboardButtonKind::CallbackData(
            if is_public {
                EDIT_PUBLICITY_FALSE_CALLBACK.to_string()
            } else {
                EDIT_PUBLICITY_TRUE_CALLBACK.to_string()
            }
        ),
    );
    let refill_again_btn = InlineKeyboardButton::new(
        "Редактировать",
        InlineKeyboardButtonKind::CallbackData(REFILL_EVENT_AGAIN_CALLBACK.to_string()),
    );
    let publish_btn = InlineKeyboardButton::new(
        if is_public { "Опубликовать" } else { "Создать" },
        InlineKeyboardButtonKind::CallbackData(CREATE_EVENT_CALLBACK.to_string()),
    );

    let mut buttons = vec![];

    if let Some(map_link) = map_link {
        let map_link_btn = InlineKeyboardButton::new(
            "Место на карте",
            InlineKeyboardButtonKind::Url(map_link.parse().unwrap()),
        );

        buttons.push(vec![map_link_btn]);
    }

    buttons.extend([
        vec![edit_publicity_btn, refill_again_btn],
        vec![publish_btn],
    ]);

    let keyboard = InlineKeyboardMarkup::new(buttons);
    // ReplyMarkup::InlineKeyboard(keyboard)
    keyboard
}

pub fn get_inline_kb_event_message(map_link: Option<String>) -> InlineKeyboardMarkup {
    let mut buttons = vec![];

    if let Some(map_link) = map_link {
        let map_link_btn = InlineKeyboardButton::new(
            "Карта",
            InlineKeyboardButtonKind::Url(map_link.parse().unwrap()),
        );

        buttons.push(vec![map_link_btn]);
    }

    let keyboard = InlineKeyboardMarkup::new(buttons);
    keyboard
}