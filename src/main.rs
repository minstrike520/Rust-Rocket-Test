#[macro_use] extern crate rocket;


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


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,get_demo,hello])
}
