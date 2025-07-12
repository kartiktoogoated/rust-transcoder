use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL not set")
}
