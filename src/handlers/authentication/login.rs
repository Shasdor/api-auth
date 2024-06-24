use diesel::types::Json;
use rocket_contrib::json::JsonValue;
use crate::database;
use crate::database::{AuthorizationDatabase, AuthorizationOutcome};
use crate::database::authorization::LoginResponse;


pub enum LoginError {
    NotFound,
    Other,
}

pub fn login(login: &str, password: &str, db: database::Conn) -> Result<rocket_contrib::json::JsonValue, LoginError> {
    match db.login(login, password) {
        AuthorizationOutcome::Ok(s) => Ok(s),
        AuthorizationOutcome::NotFound => Err(LoginError::NotFound),
        AuthorizationOutcome::Other => Err(LoginError::Other),
    }
}