use rocket_contrib::Json;
use rocket::request::State;
use models::User;
use model::Model;
use model::Error;

#[get("/users")]
fn get(model:State<Model>) -> Result<Json<Vec<User>>,Error> {
//    model.users().map(|user| Json(user))
    model.users().map(Json)
}
