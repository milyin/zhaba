use super::schema::user;
use super::schema::post;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: i64,
}

#[derive(Queryable, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: i64,
}

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub created: i64,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub created: i64,
    pub title: &'a str,
    pub body: &'a str,
}
