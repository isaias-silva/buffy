use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
#[derive(Serialize, Deserialize)]

pub struct Response {
    message: String,
    payload:String,
}

impl Response {
    pub fn new<T:Serialize>(message: &str, data:Option<T>) -> Response {
       
        let payload = match data {
            
            Some(d) => serde_json::to_string(&d).unwrap(),
            None => String::new(),
        };
        Response {
            message: message.to_string(),
            payload
        }
    }
    pub fn to_json(&self) -> Value {
        let model = Response {
            message: self.message.to_string(),
            payload:self.payload.clone()
        };
        serde_json::json!(&model)
    }
}
