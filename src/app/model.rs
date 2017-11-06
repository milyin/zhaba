use std::sync::atomic::{AtomicUsize, Ordering};
use diesel;
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
use super::db::schema::user_infos;
use super::db::schema::posts;
use super::error::ModelError;
use super::db::conn::Pool;
use super::db::conn::init_pool;
use super::db::models::NewUser;
use super::db::models::NewPost;
use super::db::models::User;
use super::db::models::UserInfo;
use super::db::models::Post;
use super::settings::ENV;
use super::settings::AUTH_TOKEN_NAME;
use kit::hash;
use rocket::http::{Cookie, Cookies};

pub struct Model {
    pool: Pool,
    counter: AtomicUsize,
}

#[derive(Hash, Debug)]
struct TokenHash<'a> {
    user_id: i32,
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
    user_id: i32,
    expires: i64,
    hash: u64,
}

#[derive(Serialize)]
pub struct AuthInfo {
    user_id: i32,
    expires: i64,
}

pub type ModelResult<T> = Result<T, ModelError>;

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

    pub fn user(&self, name: &str) -> ModelResult<UserInfo> {
        let conn = self.pool.get()?;
        user_infos::table
            .filter(user_infos::name.eq(name))
            .limit(1)
            .load::<UserInfo>(&*conn)?
            .pop()
            .ok_or(ModelError::UserNotFound)
    }

    pub fn users(&self) -> ModelResult<Vec<UserInfo>> {
        let conn = self.pool.get()?;
        Ok(user_infos::table.load::<UserInfo>(&*conn)?)
    }
    pub fn register(&self, name: &str, email: &str, password: &str) -> ModelResult<()> {
        let conn = self.pool.get()?;
        let user = users::table
            .filter(users::name.eq(name))
            .limit(1)
            .load::<User>(&*conn)?;
        if user.len() > 0 {
            return Err(ModelError::UserExists);
        };
        let password_hash = hash(&PasswordHash {
            name: name,
            password: password,
            secret: ENV.secret,
        }) as i64;
        diesel::insert(&NewUser {
            name: name,
            email: email,
            password_hash: password_hash,
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
        let user: User = users::table
            .filter(users::name.eq(name))
            .limit(1)
            .load::<User>(&*conn)?
            .pop()
            .ok_or(ModelError::UserNotFound)?;
        let password_hash = hash(&PasswordHash {
            name: name,
            password: password,
            secret: ENV.secret,
        }) as i64;
        if user.password_hash != password_hash {
            return Err(ModelError::PasswordWrong);
        };
        let expires = time::now_utc().to_timespec().sec + duration as i64;
        let token_hash = hash(&TokenHash {
            user_id: user.id,
            expires: expires,
            extra_data: extra_data,
            secret: ENV.secret,
        });
        Ok(AuthToken {
            user_id: user.id,
            expires: expires,
            hash: token_hash,
        })
    }
    pub fn authorize(token: AuthToken, extra_data: &str) -> ModelResult<AuthInfo> {
        if time::now_utc().to_timespec().sec > token.expires {
            return Err(ModelError::AuthTokenExpired);
        };
        let token_hash = hash(&TokenHash {
            user_id: token.user_id,
            expires: token.expires,
            extra_data: extra_data,
            secret: ENV.secret,
        });
        if token.hash != token_hash {
            return Err(ModelError::AuthTokenInvalid);
        }
        Ok(AuthInfo {
            user_id: token.user_id,
            expires: token.expires,
        })
    }

    pub fn get_post(&self, post_id: i32) -> ModelResult<Post> {
        let conn = self.pool.get()?;
        posts::table
            .filter(posts::id.eq(post_id))
            .limit(1)
            .load::<Post>(&*conn)?
            .pop()
            .ok_or(ModelError::PostNotFound)
    }

    pub fn new_post(&self, auth: &AuthInfo, title: &str, body: &str) -> ModelResult<()> {
        let conn = self.pool.get()?;
        let timestamp = time::now_utc().to_timespec().sec;
        diesel::insert(&NewPost {
            user_id: auth.user_id,
            created: timestamp,
            edited: timestamp,
            title: title,
            body: body,
        }).into(posts::table)
            .execute(&*conn)?;
        Ok(())
    }
    pub fn edit_post(
        &self,
        auth: &AuthInfo,
        post_id: i32,
        title: &str,
        body: &str,
    ) -> ModelResult<()> {
        let conn = self.pool.get()?;
        let post = self.get_post(post_id)?;
        if post.user_id != auth.user_id {
            return Err(ModelError::AccessDenied);
        }; // TODO: implement roles
        let timestamp = time::now_utc().to_timespec().sec;
        diesel::update(posts::table)
            .set(&Post {
                edited: timestamp,
                title: title.to_string(),
                body: body.to_string(),
                ..post
            })
            .execute(&*conn)?;
        Ok(())
    }
    pub fn posts(&self) -> ModelResult<Vec<Post>> {
        let conn = self.pool.get()?;
        Ok(posts::table.load::<Post>(&*conn)?)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthInfo {
    type Error = ModelError;
    fn from_request(request: &'a Request) -> request::Outcome<Self, Self::Error> {
        (|| {
             let cookie = request.cookies().get_private(AUTH_TOKEN_NAME).ok_or(
                ModelError::AuthTokenNotFound,
            )?;
             let token: AuthToken = serde_json::from_str(cookie.value())?;
             Model::authorize(token, "")
         })().into_outcome(Status::Unauthorized)
    }
}

pub fn set_auth_cookie(
    model: &Model,
    cookies: &mut Cookies,
    name: &str,
    password: &str,
    extra_data: &str,
    duration: u32,
) -> ModelResult<AuthInfo> {
    let token = model.login(name, password, extra_data, duration)?;
    cookies.add_private(Cookie::new(AUTH_TOKEN_NAME, serde_json::to_string(&token)?));
    Model::authorize(token, extra_data)
}

pub fn clear_auth_cookie(cookies: &mut Cookies) {
    cookies.remove_private(Cookie::named(AUTH_TOKEN_NAME));
}
