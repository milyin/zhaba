use dotenv::dotenv;
use std::env;
use kit::hash;

pub struct Env {
    pub database_url: String,
    pub secret: u64,
}

pub const AUTH_TOKEN_NAME: &'static str = "token";

lazy_static! {
    pub static ref ENV : Env = {
        dotenv().expect("Can't open .env");
        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
            secret: hash(&env::var("SECRET").expect("SECRET must be set in .env")),
        }
    };
}
