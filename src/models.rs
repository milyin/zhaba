#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
}
