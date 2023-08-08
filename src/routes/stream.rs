use actix_web::{web, HttpResponse};
use futures::stream::StreamExt;

#[derive(Debug, serde::Deserialize,serde::Serialize)]

pub struct StreamRequest{
   pub id: String,
 }

pub async fn stream_handler(mut payload: web::Payload,_info: web::Json<StreamRequest>) -> HttpResponse {
    while let Some(chunk) = payload.next().await {
      
        println!("Received chunk: {:?}", chunk);
    }

    HttpResponse::Ok().finish()
}