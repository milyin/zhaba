use DbConn;
use rocket::request::Form;
use maud::Markup;
use kit::form::to_form;
use models::NewUser;
use diesel;
use diesel::prelude::ExecuteDsl;
use diesel::prelude::FilterDsl;
use diesel::prelude::LoadDsl;
use diesel::QueryResult;
use diesel::ExpressionMethods;
use diesel::LimitDsl;
use rocket_contrib::Json;
use models::User;

#[derive(FromForm, Default, Serialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[get("/register")]
pub fn get() -> Markup {
    to_form(&Register::default())
}

#[post("/register", data = "<data>")]
pub fn post(conn: DbConn, data: Form<Register>) -> QueryResult<Json<usize>> {
    use schema::users::dsl::*;
    let form = data.get();
    let new_post = NewUser {
        name: &form.name,
        email: &form.email,
        password_hash: &form.password
    };
    let user = users.filter(name.eq(&form.name)).limit(1).load::<User>(&*conn)?;
    if user.len()==0 {
        diesel::insert(&new_post).into(users).execute(&*conn).map(Json)
    } else {
        Ok(Json(0))
    }
}
