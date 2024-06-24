use rocket_contrib::json::JsonValue;
use crate::database;
use crate::database::{AuthorizationDatabase, RegistrationOutcome};

pub type Token = JsonValue;

pub enum RegistrationError {
    PasswordDoNotMatch,
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn registration(login: &str, password: &str, confirmPassword:&str, db: database::Conn) -> Result<Token, RegistrationError> {
// pub fn registration(login: &str, password: &str,  db: database::Conn) -> Result<(), RegistrationError> {
    if password!=confirmPassword { return Err(RegistrationError::PasswordDoNotMatch) }
    match db.registration(login, password, confirmPassword) {
    // match db.registration(login, password, "") {
        RegistrationOutcome::Ok(s) => Ok(s),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
        _ => Err(RegistrationError::Other)
    }
}