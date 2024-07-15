use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    // Using the `context! { }` macro.
    Template::render("index", context! {
        title: "Hello, world!",
        message: "Welcome to my website!"
    })
}
#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing())
        .mount("/", routes![index])
}