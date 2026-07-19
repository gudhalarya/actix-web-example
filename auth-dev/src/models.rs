//This is the file where the models will be stored
use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct User{
    pub email:String,
    pub hash:String,
    pub created_at:chrono::NaiveDateTime
}

impl User{
    pub fn from_details<S:Into<String>,T:Into<String>>(email:S,pwd:T)->Self{
        User { email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local() }
    }
}