 use rocket_contrib::json::{Json, JsonValue};
 use serde::Serialize;
 //use rocket::serde::json::Json;

use crate::database;
 use crate::handlers::authentication;
use crate::handlers::authentication::login::LoginError;
use crate::handlers::authentication::registration::RegistrationError;
use crate::routes::route_objects::ApiResponse;
use crate::routes::route_objects::error_response::{ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND, ERROR_WEAK_PASSWORD, ERROR_WRONG_REQUEST, PASSWORD_DO_NOT_MATCH};
use crate::routes::route_objects::login_request::LoginRequest;
use crate::routes::route_objects::registration_request::RegistrationRequest;
 pub type Token = JsonValue;
 #[derive(Serialize)]
 pub struct LoginResponse{
     pub token:String
 }

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub fn login(maybe_login_request: Option<Json<LoginRequest>>, db: database::Conn) -> ApiResponse<'static, Token> {
    let call_chain = maybe_login_request
        .map(|r| authentication::login::login(r.login, r.password, db));
    return match call_chain {
        Some(Ok(token)) => ApiResponse::Ok(token),
        Some(Err(LoginError::NotFound)) => ApiResponse::Err(ERROR_USER_NOT_FOUND),
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    };
}

#[post("/registration", format = "json", data = "<maybe_registration_request>")]
pub fn registration(maybe_registration_request: Option<Json<RegistrationRequest>>, db: database::Conn) -> ApiResponse<'static, Token> {
    let call_chain = maybe_registration_request
        .map(|r| authentication::registration::registration(&r.login, &r.password,&r.confirmPassword,  db));
    return match call_chain {
        Some(Ok(token)) => ApiResponse::Ok(token),
        Some(Err(RegistrationError::PasswordDoNotMatch)) => ApiResponse::Err(PASSWORD_DO_NOT_MATCH),
        Some(Err(RegistrationError::LoginInUse)) => ApiResponse::Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => ApiResponse::Err(ERROR_WEAK_PASSWORD),
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    };
}