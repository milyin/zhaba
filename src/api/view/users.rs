use rocket_contrib::Json;
use rocket::request::State;
use app::Model;
use app::ModelResult;
use app::User;

#[get("/users")]
fn get(model:State<Model>) -> Json<ModelResult<Vec<User>>> {
    Json(model.users())
}
