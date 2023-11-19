/// Internationalization
pub trait MyI18N {
    fn to_text(&self) -> &'static str;

    fn from_text(text: &str) -> Self;
}