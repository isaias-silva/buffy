pub mod user_service {
    use crate::{controllers::user::user_controller::Response, structs::body::body::Login};
    use dotenv::dotenv;
    use std::env;

    pub fn check_user(user: Login) -> Result<Response, Response> {
        dotenv().ok();
        match user {
            Login {
                password: Some(_password),
                name: Some(_name),
            } => {

            
                   let real_name= env::var("USERNAME").expect("user name not found");
                    let real_password=env::var("PASSWORD").expect("password user not found");
             
            
                let name_res=_name.clone(); 
                let password_res=_password.clone();
                if real_name==name_res && password_res==real_password{

                    print!("{} = {}",real_name,name_res);
                    return Ok(Response::new::<&str>(&format!("welcome {}!", name_res), None));
                }else{
                    return Err(Response::new::<&str>(&format!("incorrect password"), None));

                }
               
            }
            Login {
                name: Some(_name),
                password: None,
            } => {
                return Err(Response::new::<&str>("invalid password", None));
            }

            Login {
                name: None,
                password: Some(_password),
            } => {
                return Err(Response::new::<&str>("invalid name", None));
            }
            Login {
                name: None,
                password: None,
            } => {
                return Err(Response::new::<&str>("use name and password", None));
            }
        }
    }
}
