//This is where the models will be stored
use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
   pub name:String,
   pub email:String,
   pub pwd:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Login{
   pub email:String,
   pub pwd:String
}


