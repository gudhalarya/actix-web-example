use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

use crate::error::{AppError, AppResponse};

//helper fn are here dude ----------> 
pub fn hash_password(password:&str)->AppResponse<String>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pwd = argon2.hash_password(password.as_bytes(), &salt).map_err(|err|anyhow::anyhow!("Password hashing failed dude {}",err))?.to_string();
    Ok(pwd)
}