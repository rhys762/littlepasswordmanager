/*
    Anything database related.

    The general idea is there is a password the user needs to set on first use, and reenter every time the app is opened
    Stored as a hash.

    The unhashed password will be used to encrypt and decrypt all the domain and passwords.
*/

use serde::Serialize;

// denotes an entry in the database.
#[derive(Clone, Serialize)]
pub struct Password {
    // website, app etc
    pub domain: String,
    // the actuall password
    pub password: String,
}

// create the tables if they do not exist
pub fn create_tables(conn: &sqlite::Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS master_password (password STRING);

        CREATE TABLE IF NOT EXISTS passwords (
            domain STRING,
            password STRING,

            CONSTRAINT domain_pk PRIMARY KEY (domain)
        );
    ";
    conn.execute(query).expect("create_tables");
}

// get the master password hash, which the user will need to test against.
// empty if not set, which should only be the case if the db has never been used.
pub fn get_master_passwordhash(conn: &sqlite::Connection) -> String {
    let query = "SELECT * FROM master_password";

    // print!("running query '{query}'\n");

    let mut hash = String::new();

    conn.iterate(query, |pairs| {
        for &(_name, value) in pairs.iter() {
            hash = String::from(value.expect("Could not extract master password hash."));
        }
        true
    })
    .expect("get_master_passwordhash:iterate");

    hash
}

// set the master hash, and return it.
pub fn set_master_passwordhash(conn: &sqlite::Connection, master_hash: &String) {
    let query = "INSERT INTO master_password VALUES (?)";

    let mut query = conn.prepare(query).expect("set_master_passwordhash::prepare");
    query.bind((1, master_hash.as_str())).expect("set_master_passwordhash::bind");
    query.next().expect("set_master_passwordhash::next");
}

// inserts if domain already is present, else update.
pub fn update_password(conn: &sqlite::Connection, entry: &Password) {
    let query = "
        INSERT OR REPLACE INTO passwords (domain, password)
        VALUES (:domain, :password)
    ";

    let mut query = conn.prepare(query).expect("update_password::prepare");
    query.bind((":domain", entry.domain.as_str())).expect("update_password::domain");
    query.bind((":password", entry.password.as_str())).expect("update_password::passpord");

    query.next().unwrap();
}

// remove a entry from the database.
pub fn delete_password(conn: &sqlite::Connection, domain: &String) {
    let query = "DELETE FROM passwords WHERE domain=?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, domain.as_str())).unwrap();
    statement.next().unwrap();
}

// retrieve all the passwords
pub fn get_passwords(conn: &sqlite::Connection) -> Vec<Password> {
    let mut vec: Vec<Password> = Vec::new();

    let query = "SELECT * from passwords;";

    conn.iterate(query, |pairs| {
        let mut domain = "";
        let mut password = "";

        for &(name, value) in pairs.iter() {
            if name == "domain" {
                domain = value.expect("get_passwords::domain");
            } else if name == "password" {
                password = value.expect("get_passwords::password");
            }
        }
        let p = Password {
            domain: String::from(domain),
            password: String::from(password)
        };
        vec.push(p);

        true
    })
    .expect("get_passwords::iterate");

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> sqlite::Connection {
        let db = sqlite::open(":memory:").unwrap();
        create_tables(&db);
        return db;
    }

    #[test]
    fn it_should_set_and_get_master_password() {
        let db = create_test_db();

        let master_password = "this is the master password.";
        let hash = format!("@@{}@@", master_password);

        set_master_passwordhash(&db, &hash);
        let retrieved_password = get_master_passwordhash(&db);

        assert_eq!(retrieved_password, hash);
    }

    #[test]
    fn it_should_update_passwords() {
        // insert
        let db = create_test_db();

        update_password(&db, &Password {
            domain: String::from("testDomain"),
            password: String::from("testPassword")
        });

        let v = get_passwords(&db);
        assert_eq!(v.len(), 1);

        assert_eq!(v[0].domain.to_string(), "testDomain");
        assert_eq!(v[0].password.to_string(), "testPassword");

        // update, check overwritten
        
        update_password(&db, &Password {
            domain: String::from("testDomain"),
            password: String::from("updatedPassword")
        });

        let v = get_passwords(&db);

        assert_eq!(v.len(), 1);

        assert_eq!(v[0].domain.to_string(), "testDomain");
        assert_eq!(v[0].password.to_string(), "updatedPassword");

        // add another, check not overwritten

        update_password(&db, &Password {
            domain: String::from("testDomain2"),
            password: String::from("secondPassword")
        });

        let v = get_passwords(&db);

        assert_eq!(v.len(), 2);

        assert_eq!(v[0].domain.to_string(), "testDomain");
        assert_eq!(v[0].password.to_string(), "updatedPassword");

        assert_eq!(v[1].domain.to_string(), "testDomain2");
        assert_eq!(v[1].password.to_string(), "secondPassword");
    }

    #[test]
    fn it_should_delete_passwords() {
        let db = create_test_db();

        update_password(&db, &Password {
            domain: String::from("a"),
            password: String::from("a")
        });

        update_password(&db, &Password {
            domain: String::from("b"),
            password: String::from("b")
        });

        let v = get_passwords(&db);

        assert_eq!(v.len(), 2);

        delete_password(&db, &String::from("a"));

        let v = get_passwords(&db);

        assert_eq!(v.len(), 1);
        assert_eq!(v[0].domain, "b");
        assert_eq!(v[0].password, "b");
    }

    // #[test]
    // fn it_should_update_delete_and_get_passwords() {

    // }
}
