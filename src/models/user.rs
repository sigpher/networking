pub mod user {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User<'a> {
        pub username: &'a str,
        pub age: u8,
    }
}
