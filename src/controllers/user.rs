pub mod user {
    pub use crate::structs::response::Response;
    use warp::{hyper::StatusCode, Filter};

    static PATH: &str = "user";
    pub fn me() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path(PATH).and(warp::path("hello")).map(|| {
           
           
           
            warp::reply::with_status(
                warp::reply::json(&Response::new("user info", {}).to_json()),
                StatusCode::ACCEPTED,
            )
        })
    }
}
