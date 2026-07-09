use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{Error::Password, SaltString}};
use anyhow::Context;
use crate::error::AppResponse;

//These are the fn that will help in the login and signup 
pub fn hash_password(password:&str)->AppResponse<String>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_pass = argon2.hash_password(&password.as_bytes(), &salt).map_err(|err|anyhow::anyhow!(err)).context("Hashing failed")?.to_string();
    Ok(hashed_pass)
}

//This is to verify the password
pub fn verify_password(password:&str,hash:&str)->AppResponse<bool>{
    let parsed_hash = PasswordHash::new(hash).map_err(|err|anyhow::anyhow!(err)).context("Wrong hash is there ")?;
    let argon2 = Argon2::default();
    match argon2.verify_password(&password.as_bytes(), &parsed_hash){
        Ok(_)=>Ok(true),
        Err(argon2::password_hash::Error::Password)=>Ok(false),
        Err(e)=>{
            Err(anyhow::anyhow!(e).context("Failed to verify the pass"))?
        }
    }
}