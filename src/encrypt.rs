use crate::database;

use age::secrecy::Secret;
use std::io::{Read, Write};

pub fn hash_password(password: &String) -> String {
    return sha256::digest(password);
}

fn encrypt(master: &String, raw: &String) -> String {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(master.to_owned()));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).expect("");
    writer.write_all(raw.as_bytes());
    writer.finish();

    // return as space seperated numbers.
    let mut out = String::new();

    encrypted.iter().map(|b| {
        b.to_string()
    })
    .for_each(|s| {
        out.push_str(&s);
        out.push_str(" ");
    });

    out
}

fn decrypt(master: &String, encrypted: &String) -> String {
    let encrypted = Vec::from_iter(encrypted.split(" ")
    .filter(|str| {
        str.len() > 0
    })
    .map(|str| {
        // HERE
        str.parse::<u8>().unwrap()
    }));

    let decryptor = match age::Decryptor::new(&encrypted[..]).expect("") {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(master.to_owned()), None).expect("");
    reader.read_to_end(&mut decrypted);

    String::from_utf8(decrypted).expect("")
}

pub fn encrypt_password(master: &String, password: &database::Password) -> database::Password {
    return database::Password {
        domain: encrypt(master, &password.domain),
        password: encrypt(master, &password.password)
    };
}

pub fn decrypt_password(master: &String, password: & database::Password) -> database::Password {
    return database::Password {
        domain: decrypt(master, &password.domain),
        password: decrypt(master, &password.password)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_encrypt() {
        let master = String::from("THIS IS A PASSWORD.");
        let raw = String::from("THIS IS TO BE ENCRYPTED AND THEN DECRYPTED.");

        let encryped = encrypt(&master, &raw);
        let decrypted = decrypt(&master, &encryped);

        assert_eq!(raw, decrypted);
    }
}