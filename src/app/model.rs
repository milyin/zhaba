use std::sync::atomic::{AtomicUsize, Ordering};
use diesel;
use diesel::prelude::SelectDsl;
use diesel::prelude::LoadDsl;
use diesel::prelude::LimitDsl;
use diesel::prelude::ExecuteDsl;
use diesel::prelude::FilterDsl;
use diesel::ExpressionMethods;
use rocket::request;
use rocket::request::Request;
use rocket::request::FromRequest;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use time;
use serde_json;
use super::db::schema::users;
use super::error::ModelError;
use super::error::ModelResult;
use super::db::conn::Pool;
use super::db::conn::init_pool;
use super::db::models::NewUser;
use super::db::models::UserFull;
use super::User;
use super::settings::ENV;
use super::settings::COOKIE_TOKEN;
use kit::hash;

pub struct Model {
    pool: Pool,
    counter: AtomicUsize,
}

#[derive(Hash, Debug)]
struct TokenHash<'a> {
    name: &'a str,
    expires: i64,
    extra_data: &'a str,
    secret: u64,
}

#[derive(Hash)]
struct PasswordHash<'a> {
    name: &'a str,
    password: &'a str,
    secret: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    name: String,
    expires: i64,
    hash: u64,
}

#[derive(Serialize)]
pub struct AuthInfo {
    name: String,
    expires: i64,
}

impl Model {
    pub fn new() -> Model {
        Model {
            pool: init_pool(&ENV.database_url),
            counter: AtomicUsize::new(0),
        }
    }
    pub fn inc(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn user(&self, name: &str) -> ModelResult<User> {
        let conn = self.pool.get()?;
        users::table
            .select((users::name, users::email))
            .filter(users::name.eq(name))
            .limit(1)
            .load::<User>(&*conn)?
            .pop()
            .ok_or(ModelError::UserNotFound)
    }

    pub fn users(&self) -> ModelResult<Vec<User>> {
        let conn = self.pool.get()?;
        Ok(users::table
            .select((users::name, users::email))
            .load::<User>(&*conn)?)
    }
    pub fn register(&self, name: &str, email: &str, password: &str) -> ModelResult<()> {
        let conn = self.pool.get()?;
        let user = users::table
            .filter(users::name.eq(name))
            .limit(1)
            .load::<UserFull>(&*conn)?;
        if user.len() > 0 {
            return Err(ModelError::UserExists);
        };
        let password_hash = hash(&PasswordHash {
            name: name,
            password: password,
            secret: ENV.secret,
        }).to_string();
        diesel::insert(&NewUser {
            name: name,
            email: email,
            password_hash: &password_hash,
        }).into(users::table)
            .execute(&*conn)?;
        Ok(())
    }
    pub fn login(
        &self,
        name: &str,
        password: &str,
        extra_data: &str,
        duration: u32,
    ) -> ModelResult<AuthToken> {
        let conn = self.pool.get()?;
        let user: UserFull = users::table
            .filter(users::name.eq(name))
            .limit(1)
            .load::<UserFull>(&*conn)?
            .pop()
            .ok_or(ModelError::UserNotFound)?;
        let password_hash = hash(&PasswordHash {
            name: name,
            password: password,
            secret: ENV.secret,
        }).to_string();
        if user.password_hash != password_hash {
            return Err(ModelError::PasswordWrong);
        };
        let expires = time::now_utc().to_timespec().sec + duration as i64;
        let token_hash = hash(&TokenHash {
            name: name,
            expires: expires,
            extra_data: extra_data,
            secret: ENV.secret,
        });
        Ok(AuthToken {
            name: name.to_string(),
            expires: expires,
            hash: token_hash,
        })
    }
    pub fn authorize(token: AuthToken, extra_data: &str) -> ModelResult<AuthInfo> {
        if time::now_utc().to_timespec().sec > token.expires {
            return Err(ModelError::AuthTokenExpired);
        };
        let token_hash = hash(&TokenHash {
            name: &token.name,
            expires: token.expires,
            extra_data: extra_data,
            secret: ENV.secret,
        });
        if token.hash != token_hash {
            return Err(ModelError::AuthTokenInvalid);
        }
        Ok(AuthInfo {
            name: token.name,
            expires: token.expires,
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthInfo {
    type Error = ModelError;
    fn from_request(request: &'a Request) -> request::Outcome<Self, Self::Error> {
        (|| {
             let cookie = request.cookies().get_private(COOKIE_TOKEN).ok_or(
                ModelError::AuthTokenNotFound,
            )?;
             let token: AuthToken = serde_json::from_str(cookie.value())?;
             Model::authorize(token, "")
         })().into_outcome(Status::Unauthorized)
    }
}
