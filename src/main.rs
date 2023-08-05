
const PORT: u16 = 3030;

/*#modules */
mod controllers;
mod structs;
mod services;
/* */
use controllers::user::user_controller;
use structs::response::Response;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello").and(warp::get()).map(|| {
        let response = Response::new::<&str>("hello world",None);

        warp::reply::json(&response.to_json())
    });
    let routes = user_controller::routes()
    .or(hello);

    (|| {
        println!("Server on in {}", PORT);
    })();

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}
