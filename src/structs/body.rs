pub mod body{
    #[derive(Debug, serde::Deserialize,serde::Serialize)]
   
    pub struct Login{
      pub  name:Option<String>,
      pub  password:Option<String>
    }
    impl Login{
        pub fn _new(name:&str,password:&str)->Login{
            Login{
                name:Some(name.to_string()),
                password:Some(password.to_string())
            }
        }
        pub fn _get(self)->Login{
            self
        }
    }
}