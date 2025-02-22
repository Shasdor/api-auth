use rocket::{Request, Response};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket_contrib::json::Json;

//use rocket::serde::json::{Deserialize, json};

#[derive(Copy, Clone)]
pub struct ErrorResponse<'a> {
    cause: &'a str,
    status: Status,
}


impl<'r> Responder<'r> for ErrorResponse<'r> {
    fn respond_to(self, request: &Request) -> Result<'r> {
         if let Ok(response) = Json(json!({"error": self.cause})).respond_to(request) {
        //if let Ok(response) = (json!({"error": self.cause})).respond_to(request) {
            Response::build_from(response).status(self.status).header(ContentType::JSON).ok()
        } else {
            Response::build().status(Status::InternalServerError).header(ContentType::JSON).ok()
        }
    }
}


// common errors
pub const ERROR_UNKNOWN: &'static ErrorResponse<'static> = &ErrorResponse { cause: "unknown", status: Status::InternalServerError };
pub const ERROR_WRONG_REQUEST: &'static ErrorResponse<'static> = &ErrorResponse { cause: "wrong_request", status: Status::BadRequest };

// login error
pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse { cause: "user_not_found", status: Status::BadRequest };

// registration error
pub const ERROR_WEAK_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse { cause: "weak_password", status: Status::BadRequest };
pub const PASSWORD_DO_NOT_MATCH: &'static ErrorResponse<'static> = &ErrorResponse { cause: "password_do_not_match", status: Status::BadRequest };

pub const ERROR_ALREADY_REGISTERED: &'static ErrorResponse<'static> = &ErrorResponse { cause: "already_registered", status: Status::BadRequest };