
fn main() {
    // despite rocket's warning, let's retain control over our own main funciton, thank-you.
    //   There are smaller-code options #[launch] and #[rocket::main] from rocket.
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let _ = runtime.block_on(hero::http::webserver::start());
}

