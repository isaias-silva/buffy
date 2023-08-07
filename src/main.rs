use actix_web::*;
use dotenv::dotenv;
use std::env;


#[actix_web::main]

async fn main() -> Result<(), std::io::Error> {
    
    dotenv().ok();
    let server = 
    HttpServer::new(|| App::new());

    let port = env::var("PORT").expect("error env var not found in .env file");

    let api = server
        .bind(format!("127.0.0.1:{}", port))
        .expect("error in connecting");
    println!("server run in port {}", port);
    api.run().await
}
