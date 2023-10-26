use rocket::{
    get,
    post,
    Rocket,
    Ignite,
    routes
};

use crate::services::superheroes::spiderman;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/spiderman/attack/<villain>")]
fn spiderman_thwip(villain: &str) -> String {
    spiderman::thwip(villain)
}

pub async fn start_webserver() -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/superheroes", routes![spiderman_thwip]);

    rocket.launch().await
}

