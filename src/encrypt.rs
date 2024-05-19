use crate::database;

// TODO

pub fn hash_password(password: &String) -> String {
    format!("hashed({password})")
}

pub fn encrypt_password(master: &String, password: &mut database::Password) {
}

pub fn decrypt_password(master: &String, password: &mut database::Password) {
}