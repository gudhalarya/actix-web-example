//This is where the models will be stored
use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    name:String,
    email:String,
    pwd:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Login{
    email:String,
    pwd:String
}


