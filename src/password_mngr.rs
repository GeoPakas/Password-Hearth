use rusqlite::Connection;
use crate::database::{self, verify_master_password_hash};
use std::path::Path;
use argon2::{
    password_hash::{PasswordHasher, SaltString
    },
    Argon2
};



pub struct PasswordManager {
    conn: Connection,
    #[allow(dead_code)] // remove when fixed
    master_password_hash: Option<String>, // you can cache it for later use and remove this warning
}

impl PasswordManager {
    pub fn new() -> color_eyre::eyre::Result<Self> {
        let db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/password_manager.db");
        let conn = Connection::open(db_path)?;
        Ok(Self { 
            conn,
            master_password_hash: None, 
        })
    }
    
    pub fn signup(&self) -> color_eyre::eyre::Result<()> {
        use rand_core::OsRng; // Secure random number generator
        
        let username = self.get_input("Enter a username: ");
        let master_password = self.get_input("Enter a master password");

        let salt = SaltString::generate(&mut OsRng);
        
        let argon2 = Argon2::default();
        let hashed_master_password = argon2
            .hash_password(master_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        database::add_new_signup_user(&self.conn, &username, &hashed_master_password)?;

        println!("Account created successfully!");
        Ok(())
    }

    pub fn login(&mut self) -> color_eyre::eyre::Result<()> {
        let username = self.get_input("Enter your username: ");
        let entered_password = self.get_input("Enter your password: ");


        if let Some(hash) = database::fetch_master_password_from_db(&self.conn, &username)? {
            self.master_password_hash = Some(hash.clone()); // Cache the hash
            
            if verify_master_password_hash(&hash, &entered_password) {
                println!("Login successfull!");
            } else {
                println!("Invalid password!");
            }
        } else {
            println!("User not found!");
        }
        Ok(())
    }

    fn get_input(&self, prompt: &str) -> String {
        println!("{}", prompt);
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    }

    
    
}