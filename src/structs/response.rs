use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
#[derive(Serialize, Deserialize)]

pub struct Response {
    message: String,
    payload:String,
}

impl Response {
    pub fn new<T:Serialize>(message: &str,data:T) -> Response {
        Response {
            message: message.to_string(),
            payload:serde_json::to_string(&data).unwrap()
        }
    }
    pub fn to_json(&self) -> Value {
        let model = Response {
            message: self.message.to_string(),
            payload:self.payload.to_string()
        };
        serde_json::json!(&model)
    }
}
