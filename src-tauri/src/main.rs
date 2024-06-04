// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ ops::Deref, sync::Mutex};

mod database;
mod encrypt;
mod levenshtein;

// app state
struct LPMState {
    db_path: String,
    // master password
    password: Mutex<String>,
    // the list of unencrypted passwords
    passwords: Mutex<Vec<database::Password>>
}

#[tauri::command]
fn log(s: &str) {
    println!("{s}\n");
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

// async makes it not run on the main thread
#[tauri::command(async)]
fn get_passwords(state: tauri::State<LPMState>, filter: &str) -> Vec<database::Password> {
    let conn = sqlite::open(state.db_path.clone()).expect("get_passwords::conn");
    let password = state.password.lock().expect("get_passwords::password");

    let mut passwords = state.passwords.lock().expect("get_passwords::passwords");

    if passwords.len() == 0 {
        // haven't loaded, load
        *passwords = Vec::from_iter(database::get_passwords(&conn).iter().map(|p| {
            return encrypt::decrypt_password(&password, p);
        }));
    }

    let passwords = passwords.clone();

    if filter.len() == 0 {
        return passwords;
    }

    let mut passwords = Vec::from_iter(passwords.iter().map(|p| {
        (p, levenshtein::levenshtein(&String::from(filter), &p.domain))
    }));

    passwords.sort_by(|a, b| {
        return a.1.cmp(&b.1);
    });

    return  Vec::from_iter(passwords.iter().map(|pair| {
        return pair.0.clone();
    })); 


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

#[tauri::command]
fn delete_password(state: tauri::State<LPMState>, domain: &str) {
    let password = state.password.lock().expect("get_passwords::password");
    
    // remove from local state
    let mut passwords = state.passwords.lock().expect("get_passwords::passwords");
    *passwords = Vec::from_iter(passwords.iter().filter(|p| {
        p.domain != domain
    }).map(|p| {
        p.clone()
    }));

    // remove from db:
    // re-get from db. AES apparently does not salt, but kept getting different values when
    // trying to go the other way around. TODO, would be nice to not re-unencrypt the whole db
    // every time a password is deleted. Maybe its a trailing space somewhere.
    let db_path = state.db_path.clone();
    let password = String::from(password.deref());
    let domain = String::from(domain);
    std::thread::spawn (move || {
        let conn = sqlite::open(db_path).expect("delete_password::conn");
        let passwords = database::get_passwords(&conn);

        let mut pairs = passwords.iter().map(|p| {
            return (p, encrypt::decrypt_password(&password, p));
        });

        let to_delete = pairs.find(|p| {
            p.1.domain == domain
        });

        match to_delete {
            Some(x) => {
                let key = &x.0.domain;
                database::delete_password(&conn, &key);
            },
            // Not in db?
            None => {}
        } 
    });   
}

fn main() {
    let default_name = ".little_password_manager.sqlite3";
    let db_path = match std::env::var("LPM_DB_PATH") {
        Ok(v) => String::from(v),
        Err(_) => match std::env::var("HOME") {
            Ok(v) => format!("{}/{}", v, default_name),
            Err(_) => format!("./{}", default_name),
        },
    };

    let conn = sqlite::open(db_path.clone()).expect("main::conn");
    
    database::create_tables(&conn);

    let state = LPMState {
        db_path: String::from(db_path),
        password: String::new().into(),
        passwords: Vec::new().into()
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            log,
            get_master_hash,
            setup_master_password,
            get_passwords,
            login,
            create_password,
            generate_password,
            delete_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
