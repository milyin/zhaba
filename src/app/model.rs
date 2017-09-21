use std::sync::atomic::{AtomicUsize, Ordering};
use diesel;
use diesel::prelude::LoadDsl;
use diesel::prelude::LimitDsl;
use diesel::prelude::ExecuteDsl;
use diesel::prelude::FilterDsl;
use diesel::ExpressionMethods;
use super::db::schema::users;
use super::error::ModelError;
use super::error::ModelResult;
use super::db::conn::Pool;
use super::db::conn::init_pool;
use super::db::models::NewUser;
use super::*;

pub struct Model {
    pool: Pool,
    counter: AtomicUsize
}

impl Model {

    pub fn new() -> Model {
        Model {
            pool: init_pool(),
            counter: AtomicUsize::new(0)
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
        if user.len() > 0 {
            Err(ModelError::UserExists)
        } else {
            diesel::insert(&NewUser {
                name: name,
                email: email,
                password_hash: password
            }).into(users::table).execute(&*conn)?;
            Ok(())
        }
    }
}

