pub mod user_service {
    use crate::{controllers::user::user_controller::Response, structs::body::body::Login};

    pub fn check_user(user: Login) -> Result<Response, Response> {
      
        match user {
            Login {
                password: Some(_password),
                email: Some(_email),
            } => {
                return Ok(Response::new::<&str>("user is root", None));
            },
            Login { email: Some(_email), password:None } => {
               
                return Err(Response::new::<&str>("invalid password", None)); 
            }
           
            Login { email: None, password:Some(_password) } => {
                return Err(Response::new::<&str>("invalid email", None));
               
            }
            Login { email: None, password:None} => {
                return Err(Response::new::<&str>("use email and password", None));
               
            }
        }
    }
}
