use DbConn;
use rocket_contrib::Json;
use diesel::QueryResult;
use models::User;
use schema::users::dsl::*;
use diesel::prelude::LoadDsl;

#[get("/users")]
fn get(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    users.load::<User>(&*conn).map(|user| Json(user))
}
