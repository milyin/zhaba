use rocket_contrib::Json;
use rocket::http::{Cookie, Cookies};
use maud::Markup;
use kit::form::to_form;
use app::ModelResult;

#[derive(FromForm, Default, Serialize)]
pub struct Logout {
}

#[get("/logout")]
pub fn get() -> Markup {
    to_form(&Logout::default())
}

#[post("/logout")]
pub fn post(
    mut cookies: Cookies,
) -> Json<ModelResult<()>> {
    cookies.remove_private(Cookie::named("token"));
    Json(Ok(()))
}
