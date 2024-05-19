#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::App;
use egui::{Color32, RichText};

mod encrypt;
mod database;
mod levenshtein;

#[derive(PartialEq)]
enum AppPage {
    CreateMasterPassword,
    Login,
    ViewPasswords,
    AddPassword
}

struct AppState {
    conn: sqlite::Connection,
    page: AppPage,
    // used for creating the master password
    set_master_password: String,
    confirm_master_password: String,
    // raw master password, used for encrypting and decrypting passwords
    master_password: String,
    // for logging in
    password: String,
    // error message to display
    error: String,
    // pattern to match to domain
    search: String,
    // most recent search result
    passwords: Vec<database::Password>,
    // for adding new password
    newDomain: String,
    newPassword: String
}

fn main() {
    // collect args
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        print!("Expects a single arguement, the path to a sqlite database, if the database does not exist it will be created.\n");
        return;
    } 

    let db_path = &args[1];

    // open db, create tables if required
    let required_end = String::from(".sqlite3");
    if !db_path.ends_with(&required_end) {
        print!("Got sqlite databse {db_path}, but must end in {required_end}.\n");
        return;
    }

    let db_conn = sqlite::open(db_path).expect("Could not open sqlite database.");

    database::create_tables(&db_conn);

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut state = AppState {
                conn: db_conn,
                page: AppPage::CreateMasterPassword,
                set_master_password: String::new(),
                confirm_master_password: String::new(),
                master_password: String::new(),
                password: String::new(),
                error: String::new(),
                search: String::new(),
                passwords: Vec::new(),
                newDomain: String::new(),
                newPassword: String::new()
            };

            // if master password already set, can go to login
            if database::get_master_passwordhash(&state.conn).len() > 0 {
                state.page = AppPage::Login
            }

            Box::<AppState>::from(state)
        }),
    );
}

// TODO
fn load_passwords(conn: &sqlite::Connection, master: &String) -> Vec<database::Password> {
    // TODO encryption
    return database::get_passwords(&conn);
}

// TODO
fn save_password(conn: &sqlite::Connection, master: &String, p: database::Password) {
    // TODO encryption
    database::update_password(&conn, &p);
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.error.len() > 0 {
                ui.label(RichText::new(&self.error).color(Color32::RED));
            }

            if self.page == AppPage::CreateMasterPassword {
                // Ui needs to set a master password.

                let label_set_master_password = ui.label("Set Password:");
                ui.text_edit_singleline(&mut self.set_master_password)
                    .labelled_by(label_set_master_password.id);

                let label_confirm_master_password = ui.label("Confirm Password:");
                ui.text_edit_singleline(&mut self.confirm_master_password)
                    .labelled_by(label_confirm_master_password.id);

                if ui.button("Submit").clicked() {
                    if self.set_master_password.len() > 0 && self.set_master_password == self.confirm_master_password {
                        database::set_master_passwordhash(&self.conn, &encrypt::hash_password(&self.set_master_password));
                        self.page = AppPage::Login;
                        self.error = String::new();
                    } else {
                        // error? TODO
                        self.error = String::from("Passwords must match and be non-empty");
                        self.confirm_master_password = String::new();
                    }
                }
            } else if self.page == AppPage::Login {
                // Ui needs to get password from user and check if its correct

                let label_password = ui.label("Enter Password:");
                ui.text_edit_singleline(&mut self.password)
                    .labelled_by(label_password.id);

                if ui.button("Submit").clicked() {
                    let hashed_password = encrypt::hash_password(&self.password);
                    let master = database::get_master_passwordhash(&self.conn);

                    if hashed_password == master {
                        self.master_password = self.password.clone();
                        self.passwords = load_passwords(&self.conn, &self.master_password);
                        self.page = AppPage::ViewPasswords;
                        self.error = String::new();
                    } else {
                        self.error = String::from("Incorrect password, try again");
                        self.password = String::new();
                    }
                }
            } else if self.page == AppPage::ViewPasswords {
                // ui needs to display passwords

                if ui.button("Create new Password").clicked() {
                    self.page = AppPage::AddPassword;
                    self.error = String::new();
                }

                let label_search = ui.label("Filter:");
                let entry = ui.text_edit_singleline(&mut self.search)
                    .labelled_by(label_search.id);
                if entry.changed() {
                    // TODO string distance etc
                    self.passwords = load_passwords(&self.conn, &self.master_password);

                    if self.search.len() > 0 {
                        let mut passwords = Vec::from_iter(self.passwords.iter().map(|p| {
                            (p, levenshtein::levenshtein(&self.search, &p.domain))
                        }));
                        
                        passwords.sort_by(|a, b| {
                            return a.1.cmp(&b.1);
                        });

                        self.passwords = Vec::from_iter(passwords.iter().map(|pair| {
                            return pair.0.clone();
                        }));
                    }
                }

                for p in &self.passwords {
                    ui.label(&p.domain);
                    ui.label(&p.password);
                }


            } else if self.page == AppPage::AddPassword {
                // ui needs to present a form for creating a new password

                if ui.button("Back").clicked() {
                    self.page = AppPage::ViewPasswords;
                    self.error = String::new();
                    return;
                }

                let label_domain = ui.label("Domain:");
                ui.text_edit_singleline(&mut self.newDomain)
                    .labelled_by(label_domain.id);

                let label_password = ui.label("Password:");
                ui.text_edit_singleline(&mut self.newPassword)
                    .labelled_by(label_password.id);

                if ui.button("Create").clicked() {
                    let p = database::Password {
                        domain: self.newDomain.clone(),
                        password: self.newPassword.clone()
                    };
                    save_password(&self.conn, &self.master_password, p);

                    self.newDomain = String::new();
                    self.newPassword = String::new();

                    self.passwords = load_passwords(&self.conn, &self.master_password);
                    self.search = String::new();

                    self.error = String::new();
                    self.page = AppPage::ViewPasswords;
                }
            }
        });
    }
}