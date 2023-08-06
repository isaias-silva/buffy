pub mod user_controller {
    use crate::{services::user::user_service::check_user, structs::body::body::Login};

    pub use crate::structs::response::Response;
    use warp::{hyper::StatusCode, Filter};

    static PATH: &str = "user";
    pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let routes = me().or(login());
        routes
    }

    pub fn me() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path(PATH)
            .and(warp::get())
            .and(warp::path("me"))
            .map(|| {
                warp::reply::with_status(
                    warp::reply::json(&Response::new::<&str>("user info", None,None).to_json()),
                    StatusCode::ACCEPTED,
                )
            })
    }
    pub fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path(PATH)
            .and(warp::post())
            .and(warp::path("login"))
            .and(warp::body::json())
            .map(|data: Login| {
                match check_user(data) {
                    Ok(response) => warp::reply::with_status(
                        warp::reply::json(&response.to_json()),
                        StatusCode::ACCEPTED,
                    ),
                    Err(err) => warp::reply::with_status(
                        warp::reply::json(&err.to_json()),
                        StatusCode::UNAUTHORIZED,
                    ),
                }
            })
    }
}
