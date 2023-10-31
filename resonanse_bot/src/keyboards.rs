use resonanse_common::models::EventSubject;
use resonanse_common::EventSubjectFilter;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup,
};

macro_rules! kb_button {
    ($s:expr) => {
        InlineKeyboardButton::new($s, InlineKeyboardButtonKind::CallbackData($s.to_string()))
    };
}

pub fn get_inline_kb_choose_subject() -> ReplyMarkup {
    let buttons = [
        vec![
            kb_button!(EventSubject::Social),
            kb_button!(EventSubject::Entertainments),
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
            kb_button!(EventSubject::Other),
            // kb_button!(EventSubject::Interests),
        ],
        // vec![
        //     kb_button!(EventSubject::Other),
        // ],
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

pub fn get_inline_kb_edit_new_event(
    is_public: bool,
    map_link: Option<String>,
) -> InlineKeyboardMarkup {
    let _edit_publicity_btn = InlineKeyboardButton::new(
        if is_public {
            "Публичное [✅]"
        } else {
            "Публичное [❌]"
        },
        InlineKeyboardButtonKind::CallbackData(if is_public {
            EDIT_PUBLICITY_FALSE_CALLBACK.to_string()
        } else {
            EDIT_PUBLICITY_TRUE_CALLBACK.to_string()
        }),
    );
    let refill_again_btn = InlineKeyboardButton::new(
        "Редактировать",
        InlineKeyboardButtonKind::CallbackData(REFILL_EVENT_AGAIN_CALLBACK.to_string()),
    );
    let publish_btn = InlineKeyboardButton::new(
        if is_public {
            "Опубликовать"
        } else {
            "Создать"
        },
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
        // vec![edit_publicity_btn, refill_again_btn],  // todo fix this button
        vec![refill_again_btn],
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

pub const EVENTS_PAGE_LEFT: &str = "EVENTS_PAGE_LEFT";
pub const EVENTS_PAGE_RIGHT: &str = "EVENTS_PAGE_RIGHT";
pub fn get_inline_kb_events_page() -> InlineKeyboardMarkup {
    let button_left = InlineKeyboardButton::new(
        "⏪ туда",
        InlineKeyboardButtonKind::CallbackData(EVENTS_PAGE_LEFT.to_string()),
    );

    let button_right = InlineKeyboardButton::new(
        "сюда ⏩",
        InlineKeyboardButtonKind::CallbackData(EVENTS_PAGE_RIGHT.to_string()),
    );

    let buttons = [vec![button_left, button_right]];

    let keyboard = InlineKeyboardMarkup::new(buttons);
    // ReplyMarkup::InlineKeyboard(keyboard)
    keyboard
}

pub const APPLY_EVENT_FILTER_BTN: &str = "APPLY_EVENT_FILTER_BTN";
pub fn get_inline_kb_set_subject_filter(
    event_filters: &EventSubjectFilter,
) -> InlineKeyboardMarkup {
    const FILTER_ON: &str = "✅";
    const FILTER_OFF: &str = "❌";
    const ROW_LEN: usize = 2;

    let max_len = event_filters
        .0
        .iter()
        .map(|(es, _)| es.to_string().len())
        .max()
        .unwrap_or(0);

    let mut buttons = event_filters
        .0
        .iter()
        .map(|(es, on)| {
            InlineKeyboardButton::new(
                format!(
                    "{}{}[{}]",
                    es.to_string(),
                    " ".repeat(max_len.saturating_sub(es.to_string().len())),
                    if *on { FILTER_ON } else { FILTER_OFF },
                ),
                InlineKeyboardButtonKind::CallbackData(es.to_string()),
            )
        })
        .collect::<Vec<_>>()
        .chunks(ROW_LEN)
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();

    let apply_button = InlineKeyboardButton::new(
        "Показать",
        InlineKeyboardButtonKind::CallbackData(APPLY_EVENT_FILTER_BTN.to_string()),
    );
    buttons.push(vec![apply_button]);

    let keyboard = InlineKeyboardMarkup::new(buttons);
    keyboard
}
