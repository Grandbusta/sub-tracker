use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");
    return password_hash.to_string();
}

pub fn compare_password(password: &str, password_hash: &str) -> bool {
    let password_hash = PasswordHash::new(password_hash)
        .expect("Failed to parse password hash");
    return Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();
}
