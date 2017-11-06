#![feature(plugin, proc_macro, custom_derive, never_type)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate maud;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
#[macro_use]
extern crate lazy_static;

use rocket::response::NamedFile;
use app::Model;
use std::io;
use std::path::{Path, PathBuf};

pub mod api;
pub mod kit;
pub mod app;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .manage(Model::new())
        .mount("/", routes![index, files])
        .mount(
            "/api",
            routes![
                api::get_show_users,
                api::get_show_posts,
                api::get_show_authinfo,
                api::get_form_login,
                api::post_form_login,
                api::get_form_logout,
                api::post_form_logout,
                api::get_form_register,
                api::post_form_register,
                api::get_form_newpost,
                api::post_form_newpost,
                api::get_form_editpost,
                api::post_form_editpost,
            ],
        )
        .launch();
}
