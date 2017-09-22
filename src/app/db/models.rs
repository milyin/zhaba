use super::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
