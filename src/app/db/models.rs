use super::schema::users;
use super::schema::posts;

#[derive(Queryable, Identifiable)]
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
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: i64,
}

#[derive(Queryable, Serialize, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub created: i64,
    pub edited: i64,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub created: i64,
    pub edited: i64,
    pub title: &'a str,
    pub body: &'a str,
}
