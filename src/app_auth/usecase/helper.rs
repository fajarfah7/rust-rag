use argon2::{Argon2, PasswordHash, PasswordVerifier};
// use argon2::{
//     password_hash::{PasswordHasher, SaltString},
// };
// use rand::rngs::OsRng;

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default(); // Argon2id, aman default

//     let hash = argon2.hash_password(password.as_bytes(), &salt)?;
//     Ok(hash.to_string())
// }
