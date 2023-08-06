pub mod user_service {
    use crate::{controllers::user::user_controller::Response, structs::body::body::Login};
    use dotenv::dotenv;
    use jsonwebtoken::{EncodingKey, encode, Header, Algorithm};
    use std::env;
    

    pub fn check_user(user: Login) -> Result<Response, Response> {
        dotenv().ok();
        match user {
            Login {
                password: Some(_password),
                name: Some(_name),
            } => {
                   let real_name= env::var("OWNERNAME").expect("user name not found");
                    let real_password=env::var("PASSWORD").expect("password user not found");
                let name_res=_name.clone(); 
                let password_res=_password.clone();
            

                if name_res==real_name && password_res==real_password{

                    let data=Login::_new(name_res.as_str(), password_res.as_str());
                    let token=generate_jwt(data).unwrap();

                    return Ok(Response::new::<&str>(&format!("welcome {}!", name_res), None,Some(token)));
                }else{
                    return Err(Response::new::<&str>(&format!("incorrect password"), None,None));

                }
               
            }
            Login {
                name: Some(_name),
                password: None,
            } => {
                return Err(Response::new::<&str>("invalid password", None,None));
            }

            Login {
                name: None,
                password: Some(_password),
            } => {
                return Err(Response::new::<&str>("invalid name", None,None));
            }
            Login {
                name: None,
                password: None,
            } => {
                return Err(Response::new::<&str>("use name and password", None,None));
            }
        }
    }
   
    fn generate_jwt(data: Login) -> Result<String, warp::Rejection> {
        dotenv().ok();
        let secret = env::var("SECRET").expect("error in get secret of env file, variable not found."); // Substitua isso pela sua chave secreta real!
        
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(secret.as_ref());
        match encode(&header, &data, &key) {
            Ok(token) => Ok(token),
            Err(_) => Err(warp::reject::reject()),
        }
    }
}
