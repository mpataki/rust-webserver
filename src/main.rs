#[macro_use] extern crate rocket;

mod services;
use services::superheroes::spiderman;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/spiderman/attack/<villain>")]
fn spiderman_thwip(villain: &str) -> String {
    spiderman::thwip(villain)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/superheroes", routes![spiderman_thwip])
}

