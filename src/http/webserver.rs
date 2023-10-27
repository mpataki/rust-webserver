use rocket::{
    get,
    post,
    Rocket,
    Ignite,
    routes
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::service::superheroes::spiderman;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello, world!")
    ),
)]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/spiderman/attack/<villain>")]
fn spiderman_thwip(villain: &str) -> String {
    spiderman::thwip(villain)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        index
    ),
)]
struct ApiDoc;

pub async fn start() -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/superheroes", routes![spiderman_thwip])
        .mount("/", SwaggerUi::new("/swagger-ui/<_..>")
               .url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

    rocket.launch().await
}

