const PORT: u16 = 3030;
mod controllers;
mod structs;
use controllers::user;
use structs::response::Response;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello").and(warp::get()).map(|| {
        let response = Response::new("hello world", {});

        warp::reply::json(&response.to_json())
    });
    let routes = user::user::me().or(hello);

    (|| {
        println!("Server on in {}", PORT);
    })();

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}
