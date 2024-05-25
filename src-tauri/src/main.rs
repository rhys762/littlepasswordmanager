// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO no unwraps

use std::sync::Mutex;
use tauri::{App, State};

mod database;
mod encrypt;

// app state
struct LPMState {
    db_path: String,
    // master password
    password: Mutex<String>,
    // the list of unencrypted passwords
    passwords: Mutex<Vec<database::Password>>
}

// retrieve master hash from db
#[tauri::command]
fn get_master_hash(state: tauri::State<LPMState>) -> String {
    let conn = sqlite::open(state.db_path.clone()).expect("getMasterHash::conn");
    let password_hash = database::get_master_passwordhash(&conn);
    return password_hash;
}

// set the master hash
#[tauri::command]
fn setup_master_password(state: tauri::State<LPMState>, password: &str, confirm_password: &str) -> String {
    let conn = sqlite::open(state.db_path.clone()).expect("setup_master_password::conn");

    if password.len() == 0 {
        return String::from("Password cannot be empty.");
    }

    if password != confirm_password {
        return String::from("Passwords must match.");
    }

    if database::get_master_passwordhash(&conn).len() > 0 {
        return String::from("Has already been set.");
    }

    database::set_master_passwordhash(&conn, &encrypt::hash_password(&String::from(password)));

    return String::from("");
}

#[tauri::command]
fn login(state: tauri::State<LPMState>, password: &str) -> String {
    let conn = sqlite::open(state.db_path.clone()).expect("login::conn");
    let master = database::get_master_passwordhash(&conn);
    let hashed_password = encrypt::hash_password(&String::from(password));

    if master != hashed_password {
        return String::from("Incorrect password, try again");
    }

    let mut p = state.password.lock().expect("login::p");
    *p = String::from(password);
    return String::new();
}

#[tauri::command]
fn get_passwords(state: tauri::State<LPMState>) -> Vec<database::Password> {
    let conn = sqlite::open(state.db_path.clone()).expect("get_passwords::conn");
    let password = state.password.lock().expect("get_passwords::password");

    let mut passwords = state.passwords.lock().expect("get_passwords::passwords");

    if passwords.len() == 0 {
        // haven't loaded, load
        *passwords = Vec::from_iter(database::get_passwords(&conn).iter().map(|p| {
            return encrypt::decrypt_password(&password, p);
        }));
    }

    return passwords.clone();
}

#[tauri::command]
fn create_password(state: tauri::State<LPMState>, domain: &str, password: &str) {
    let mut passwords = state.passwords.lock().expect("login::p");

    let p = database::Password {
        domain: String::from(domain),
        password: String::from(password)
    };

    let mut idx: i32 = -1;

    for i in 0..passwords.len() {
        if passwords[i].domain == p.domain {
            idx = i32::try_from(i).expect("could not cast idx");
            break;
        }
    }

    // update our unecrypted list
    if idx < 0 {
        // new domain
        passwords.push(p.clone());
    } else {
        // existing, overwrite.
        passwords[usize::try_from(idx).expect("could not cast usize")] = p.clone();
    }

    // encrypt, add to db.
    // encryption can take a second or two, we dont need to wait
    let db_path = state.db_path.clone();
    let master = state.password.lock().expect("create_password::master").clone();
    std::thread::spawn(move || {
        let conn = sqlite::open(db_path).expect("create_password::conn");
        let password = encrypt::encrypt_password(&master, &p);
        database::update_password(&conn, &password);
    });
}

#[tauri::command]
fn generate_password() -> String {
    return encrypt::random_password();
}

fn main() {
    let db_path = "../lpm.sqlite3";
    let conn = sqlite::open(db_path.clone()).expect("main::conn");
    
    database::create_tables(&conn);
    let hash = database::get_master_passwordhash(&conn);

    let mut state = LPMState {
        db_path: String::from(db_path),
        password: String::new().into(),
        passwords: Vec::new().into()
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_master_hash,
            setup_master_password,
            get_passwords,
            login,
            create_password,
            generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
