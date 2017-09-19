use DbConn;
use rocket_contrib::Json;
use diesel::QueryResult;
use models::User;
use diesel::prelude::LoadDsl;

#[get("/users")]
fn get(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    use schema::users::dsl::users;
    users.load::<User>(&*conn).map(|user| Json(user))
}
