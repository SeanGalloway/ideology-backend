use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    }
};

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    argon2::Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn clean_phone_number(phone_number: String) -> String {

    let digits = ['0','1','2','3','4','5','6','7','8','9'];
    let mut cleaned = String::new();

    phone_number.chars().for_each(|x| if digits.contains(&x) {cleaned.push(x)});

    cleaned
}