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
    // const subject_kb_scheme: [&[EventSubject]; 4] = [
    //     [
    //         EventSubject::Social,
    //         EventSubject::Acquaintance,
    //         EventSubject::Charity,
    //     ],
    //     [
    //         EventSubject::Culture,
    //         EventSubject::Business,
    //         EventSubject::Education,
    //     ],
    //     [
    //         EventSubject::Professional,
    //         EventSubject::Sport,
    //         EventSubject::Interests,
    //     ],
    //     [
    //         EventSubject::Other,
    //     ],
    // ];

    // InlineKeyboardButton::new(
    //             "",
    //             InlineKeyboardButtonKind::CallbackData($s.to_string()),
    //         ),


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