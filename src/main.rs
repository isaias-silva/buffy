

/*#modules */
mod controllers;
mod structs;
mod services;
/* */
use dotenv::dotenv;
use std::env;
use controllers::user::user_controller;
use structs::response::Response;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();
let port=env::var("PORT").expect("error env var not found in .env file");
    let hello =warp::get().map(|| {
        let response = Response::new::<&str>("hello world",None);

        warp::reply::json(&response.to_json())
    });
    let routes = user_controller::routes()
    .or(hello);

    (|| {
        println!("Server on in {}",port);
    })();

    warp::serve(routes).run(([127, 0, 0, 1], port.parse::<u16>().unwrap())).await;
}
