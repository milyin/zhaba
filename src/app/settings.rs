use dotenv::dotenv;
use std::env;
use kit::hash;

pub struct Settings {
    pub database_url: String,
    pub secret: u64
}

lazy_static! {
    pub static ref SETTINGS : Settings = {
        dotenv().expect("Can't open .env");
        Settings {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
            secret: hash(&env::var("SECRET").expect("SECRET must be set in .env")),
        }
    };
}