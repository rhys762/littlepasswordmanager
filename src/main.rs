#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod database;

// state of the app.
struct Appstate {

}

impl Default for Appstate {
    fn default() -> Self {
        Self {

        }
    }
}

// TODO
fn hash_password(password: &String) -> String {
    format!("hashed({password})")
}

fn main() /*-> Result<(), eframe::Error>*/ {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "My egui App",
    //     options,
    //     Box::new(|cc| {
    //         // This gives us image support:
    //         // egui_extras::install_image_loaders(&cc.egui_ctx);

    //         Box::<MyApp>::default()
    //     }),
    // )

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        print!("Expects a single arguement, the path to a sqlite database, if the database does not exist it will be created.\n");
        return;
    } 

    let db_path = &args[1];

    let required_end = String::from(".sqlite3");
    if !db_path.ends_with(&required_end) {
        print!("Got sqlite databse {db_path}, but must end in {required_end}.\n");
        return;
    }

    let db_conn = sqlite::open(db_path).expect("Could not open sqlite database.");

    database::create_tables(&db_conn);

    let master_hash = database::get_master_passwordhash(&db_conn);
    print!("DEBUG: master hash was {master_hash}\n");
    if master_hash.len() == 0 {
        // set new hash
        print!("DEBUG: Need new master password!\n");

        print!("Type password and press enter: \n");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("could not read password");

        if password.len() == 0 {
            print!("password cannot be empty!\n");
            return;
        }
        
        let hash = hash_password(&password);

        database::set_master_passwordhash(&db_conn, &hash);
        print!("DEBUG: Have set password to {password}\n");
    } else {
        // compare
    }
}

// struct MyApp {
//     name: String,
//     age: u32,
// }

// impl Default for MyApp {
//     fn default() -> Self {
//         Self {
//             name: "Arthur".to_owned(),
//             age: 42,
//         }
//     }
// }

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 let name_label = ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut self.name)
//                     .labelled_by(name_label.id);
//             });
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("Increment").clicked() {
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));

//             // ui.image(egui::include_image!(
//             //     "../../../crates/egui/assets/ferris.png"
//             // ));
//         });
//     }
// }