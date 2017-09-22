use std::sync::atomic::{AtomicUsize, Ordering};
use diesel;
use diesel::prelude::LoadDsl;
use diesel::prelude::LimitDsl;
use diesel::prelude::ExecuteDsl;
use diesel::prelude::FilterDsl;
use diesel::ExpressionMethods;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use time;
use super::db::schema::users;
use super::error::ModelError;
use super::error::ModelResult;
use super::db::conn::Pool;
use super::db::conn::init_pool;
use super::db::models::NewUser;
use super::User;

pub struct Model {
    pool: Pool,
    counter: AtomicUsize,
    secret: u64
}

#[derive(Hash)]
struct SessionData<'a> {
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

#[derive(Serialize)]
pub struct AuthToken {
    name: String,
    expires: i64,
    hash: u64
}

impl Model {

    pub fn new() -> Model {
        Model {
            pool: init_pool(),
            counter: AtomicUsize::new(0),
            secret: 0xDEADBEEF // TODO: take it from extra config
        }
    }
    pub fn inc(&self) {
        self.counter.fetch_add(1,Ordering::Relaxed);
    }
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn users(&self) -> ModelResult<Vec<User>> {
        let conn = self.pool.get()?;
        Ok(users::table.load::<User>(&*conn)?)
    }
    pub fn register(&self, name: &str, email: &str, password: &str ) -> ModelResult<()> {
        let conn = self.pool.get()?;
        let user = users::table.filter(users::name.eq(name)).limit(1).load::<User>(&*conn)?;
        if user.len() > 0 { return Err(ModelError::UserExists) };
        let mut s = DefaultHasher::new();
        PasswordHash {
            name: name,
            password: password,
            secret: self.secret,
        }.hash(&mut s);
        diesel::insert(&NewUser {
            name: name,
            email: email,
            password_hash: &s.finish().to_string(),
        }).into(users::table).execute(&*conn)?;
        Ok(())
    }
    pub fn login(&self, name: &str, password: &str, extra_data: &str, duration: i64) -> ModelResult<AuthToken> {
        let conn = self.pool.get()?;
        let user : User = users::table.filter(users::name.eq(name))
            .limit(1)
            .load::<User>(&*conn)?
            .pop()
            .ok_or(ModelError::UserNotFound)?;
        let mut s = DefaultHasher::new();
        PasswordHash {
            name: name,
            password: password,
            secret: self.secret,
        }.hash(&mut s);
        if user.password_hash != s.finish().to_string() { return Err(ModelError::PasswordWrong) };
        let expires = time::now_utc().to_timespec().sec + duration;
        SessionData {
            name: name,
            expires: expires,
            extra_data: extra_data,
            secret: self.secret,
        }.hash(&mut s);
        Ok(AuthToken{
            name: name.to_string(),
            expires: expires,
            hash: s.finish()
        })
    }
    pub fn authorize(&self, token: &AuthToken, extra_data: &str) -> ModelResult<()> {
        let mut s = DefaultHasher::new();
        if token.expires > time::now_utc().to_timespec().sec { return Err(ModelError::AuthTokenExpired)};
        SessionData {
            name: &token.name,
            expires:  token.expires,
            extra_data: extra_data,
            secret: self.secret,
        }.hash(&mut s);
        if token.hash != s.finish() { return Err(ModelError::AuthTokenInvalid)}
        Ok(())
    }
}

