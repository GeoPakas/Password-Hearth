use rusqlite::Connection;
use color_eyre::eyre::WrapErr;
use argon2::{Argon2, PasswordVerifier, PasswordHash};
use std::path::Path;
// use rand_core::OsRng; // Secure random number generator
// use rand_core::RngCore; // For generating random bytes

pub fn init_db() -> color_eyre::eyre::Result<()> {
    

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/password_manager.db");
    let conn = Connection::open(db_path)
        .wrap_err("Failed to open database.")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            master_password_hash TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            service TEXT NOT NULL,
            service_username TEXT NOT NULL,
            service_password TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;
    Ok(())
}

/*pub fn get_master_password_hash(conn: &Connection, username: &str) -> color_eyre::eyre::Result<Option<String>> {
    let mut stmt = conn
        .prepare("SELECT master_password_hash FROM users WHERE username=?1")
        .wrap_err("Failed to prepare SQL statement for retrieving master password hash.")?;

    let mut rows = stmt
        .query([username])
        .wrap_err_with(|| format!("Failed to prepare SQL statement for user '{}'", username))?;

    if let Some(row) = rows.next()? {
        let hash: String = row
            .get(0)
            .wrap_err("Failed to extract master password hash from row.")?;
        Ok(Some(hash))
    } else {
        Ok(None)
    }
}*/

pub fn fetch_master_password_from_db(conn: &Connection, username: &str) -> color_eyre::eyre::Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT master_password_hash FROM users WHERE username = ?1").wrap_err("Failed to prepare SQL statement.")?;
    let mut rows = stmt
        .query([username])
        .wrap_err_with(|| format!("Failed to prepare SQL statement for user '{}'.", username))?;

    if let Some(row) = rows.next().wrap_err("Failed to iterate over query results during verification.")? {
        let stored_hash: String = row.get(0)?;
        Ok(Some(stored_hash))
    } else {
        Err(color_eyre::eyre::eyre!("Username not found."))
    }
}

pub fn verify_master_password_hash(stored_hash: &str, entered_password: &str) -> bool {
    let argon2 = Argon2::default();
    //argon2.verify_password(entered_password.as_bytes(),stored_hash.parse().unwrap()).unwrap_or(false);
    if let Ok(parsed_hash) = PasswordHash::new(stored_hash) {
        argon2.verify_password(entered_password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}


pub fn add_new_signup_user(conn: &Connection, username: &str, master_password_hash: &str) -> color_eyre::eyre::Result<()> {
    conn.execute(
        "INSERT INTO users (username, master_password_hash) VALUES (?1, ?2)",
        [username, master_password_hash],
    )?;
    Ok(())
}
/*Encapsulate Sensitive Logic
	•	Avoid exposing raw sensitive data (like hashes) across modules unnecessarily.
	•	For example, you could move the master password verification logic into database.rs so password_manager.rs doesn’t directly handle hashes.
*/