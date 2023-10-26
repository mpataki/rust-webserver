
mod services;
mod http;
use http::webserver::start_webserver;


fn main() {
    // running this way does lead to a warning from rocket that it's running
    //   under a "custom runtime". They really want to take over the main
    //   method but I'd like to see if I can retain control over it. 
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let _ = runtime.block_on(start_webserver());
}

// This is how you'd launch a rocket webserver if that's the only thing you 
//   want your app to start up using the main method.
//#[launch]
//fn rocket() -> _ {
//    rocket::build()
//        .mount("/", routes![index])
//        .mount("/superheroes", routes![spiderman_thwip])
//}

