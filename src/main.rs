#![feature(plugin, proc_macro, custom_derive, never_type)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod api;
mod kit;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![api::update::login::get, api::update::login::post,],
        )
        .launch();
}
