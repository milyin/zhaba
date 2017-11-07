use app::{Model, ModelResult, AuthInfo, UserInfo, Post, set_auth_cookie, clear_auth_cookie};
use rocket::request::State;
use rocket_contrib::Json;
use serde::Serialize;
use rocket::http::Cookies;

#[derive(Serialize)]
pub enum ApiResult<T> where T: Serialize{
    Success(T),
    Error(String)
}

impl<T> From<ModelResult<T>> for ApiResult<T> where T: Serialize {
    fn from(r : ModelResult<T>) -> ApiResult<T> {
        r
            .map(|v| ApiResult::Success(v))
            .unwrap_or_else(|e| ApiResult::Error(e.to_string()))
    }
}

#[get("/show/users")]
pub fn get_show_users(model: State<Model>) -> Json<ApiResult<Vec<UserInfo>>> { Json(model.users().into()) }

#[get("/show/posts")]
pub fn get_show_posts(model: State<Model>) -> Json<ApiResult<Vec<Post>>> { Json(model.posts().into()) }

#[get("/show/authinfo")]
pub fn get_show_authinfo(auth: ModelResult<AuthInfo>) -> Json<ApiResult<AuthInfo>> { Json(auth.into()) }


#[derive(Default, Serialize, Deserialize)]
pub struct Login {
    name: String,
    password: String,
    duration: u32,
}

#[get("/form/login")]
pub fn get_form_login() -> Json<Login> {
    Json(Login::default())
}

#[post("/form/login", data = "<data>")]
pub fn post_form_login(model: State<Model>, data: Json<Login>, mut cookies: Cookies) -> Json<ApiResult<AuthInfo>> {
    let form = data.into_inner();
    Json(set_auth_cookie(&*model, &mut cookies, &form.name, &form.password, "", form.duration).into())
}

#[post("/form/logout")]
pub fn post_form_logout(mut cookies: Cookies) -> Json<ApiResult<()>> {
    clear_auth_cookie(&mut cookies);
    Json(ApiResult::Success(()))
}

#[derive(Default, Serialize, Deserialize)]
pub struct Register {
    name: String,
    email: String,
    password: String,
}

#[get("/form/register")]
pub fn get_form_register() -> Json<Register> {
    Json(Register::default())
}

#[post("/form/register", data = "<data>", rank=2)]
pub fn post_form_register(model: State<Model>, data: Json<Register>) -> Json<ApiResult<()>> {
    let form = data.into_inner();
    Json(model.register(&form.name, &form.email, &form.password).into())
}

#[derive(Default, Serialize, Deserialize)]
pub struct EditPost {
    post_id: i32,
    title: String,
    body: String,
}

#[get("/form/editpost")]
pub fn get_form_editpost () ->  Json<EditPost> {
    Json(EditPost::default())
}

#[post("/form/editpost", data = "<data>")]
pub fn post_form_editpost( auth: AuthInfo, model: State<Model>, data: Json<EditPost>, ) -> Json<ApiResult<()>> {
    let form = data.into_inner();
    Json(model.edit_post(&auth, form.post_id, &form.title, &form.body).into())
}

#[derive(Default, Serialize, Deserialize)]
pub struct NewPost {
    title: String,
    body: String,
}

#[get("/form/newpost")]
pub fn get_form_newpost() -> Json<NewPost> {
    Json(NewPost::default())
}

#[post("/form/newpost", data = "<data>")]
pub fn post_form_newpost(auth: AuthInfo, model: State<Model>, data: Json<NewPost>) -> Json<ApiResult<()>> {
    let form = data.into_inner();
    Json(model.new_post(&auth, &form.title, &form.body).into())
}
