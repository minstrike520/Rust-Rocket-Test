#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use std::collections::HashMap;
use rocket_dyn_templates::Template;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/demo")]
fn get_demo() -> &'static str {
    "abc"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    format!("{name},{age},{cool}")
}


#[get("/page")]
fn render_page() -> Template {
    // Create a `context` from a `HashMap`.
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("foo", "Hello, world!");
    Template::render("index",&context)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,get_demo,hello,render_page])
    .attach(Template::fairing())
}
