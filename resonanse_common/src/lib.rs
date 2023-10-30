pub use sqlx::PgPool;
pub use models::EventSubjectFilter;

pub mod models;
pub mod repository;
pub mod file_storage;

mod configuration;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
