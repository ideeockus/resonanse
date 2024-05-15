pub use clients::RecServiceClient;
pub use models::{EventSubjectFilter, RecItem, UserInteraction};
pub use sqlx::PgPool;

pub mod file_storage;
pub mod models;
pub mod repository;

mod clients;
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
