use maud::{html, Markup};

#[get("/")]
pub fn get() -> Markup {
    html! {
        h1 { "Test" }
        p { "test" }
    }
}
