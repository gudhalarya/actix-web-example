use futures_util::future::{ready,Ready};
use actix_web::FromRequest;
use uuid::Uuid;
use actix_web::{dev::Payload,HttpRequest};

//This is the extractor file here where we will do the jwt extraction and all 
pub struct AuthUser{
    pub user_id:Uuid
}

impl FromRequest for AuthUser{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self,Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_headers = match req.headers().get("Authorization") {
            Some(header)=>header, 
            None=>{
                return ready(Err(actix_web::error::ErrorUnauthorized("Missing authorization headers")));
            }
        };
        let auth_header = match auth_headers.to_str(){
            Ok(value)=>value, 
            Err(_)=>{
                return ready(Err(actix_web::error::ErrorUnauthorized("Invalid authorization headers")));
            }
        }
    }
}