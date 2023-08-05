pub mod body{
    #[derive(Debug, serde::Deserialize,serde::Serialize)]
    pub struct Login{
      pub  email:Option<String>,
      pub  password:Option<String>
    }
    impl Login{
        pub fn _new(email:&str,password:&str)->Login{
            Login{
                email:Some(email.to_string()),
                password:Some(password.to_string())
            }
        }
        pub fn _get(self)->Login{
            self
        }
    }
}