 use crate::database::{AuthorizationDatabase, Conn, AuthorizationOutcome, RegistrationOutcome};
//use crate::database::{AuthorizationDatabase,Conn, AuthorizationOutcome, RegistrationOutcome};
use crate::schema::users;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
 use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
 use rocket::http::HeaderMap;
 use rocket::http::hyper::header::Bearer;
 use rocket::http::hyper::StatusCode;
 use rocket::http::Method::Head;
 use rocket_contrib::json::Json;
 use serde::{Deserialize, Serialize};
 use serde_json::to_string;
 use uuid::Uuid;
use crate::schema::users::{secret, username};

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub secret: String,

}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
    
}
 #[derive(Serialize,Deserialize)]
 pub struct Claims{
     pub sub: String,
     pub exp: usize,
 }
 #[derive(Serialize)]
 pub struct LoginResponse{
     pub token:String
 }

impl AuthorizationDatabase for Conn {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome {
        match users::table
            .filter(users::username.eq(login.to_lowercase()))
            .get_result::<User>(&self.0) {
            Ok(user) => {
                if user.secret == password {
                    //TODO token!!!!
                    let claims = Claims{
                        sub: user.id.to_string().clone(),
                        exp: (chrono::Utc::now()+chrono::Duration::hours(1)).timestamp() as usize
                    };
                    let token = match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret("some_secret".as_ref())
                    ) {
                        Ok(tok) => tok,
                        Err(e ) => {
                            eprintln!("Error Generating Token {}",e);
                            return AuthorizationOutcome::Other
                        }
                    };
                    AuthorizationOutcome::Ok(json!({"token":token}))
                } else {
                    AuthorizationOutcome::NotFound
                }
            }
            Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
            _ => AuthorizationOutcome::Other,
        }
    }

    fn registration(&self, login: &str, password: &str,  confirmPassword: &str) -> RegistrationOutcome {
        if password!=confirmPassword{return RegistrationOutcome::DoNotMatch;}
        if password.len() < 8 {
            return RegistrationOutcome::WeakPassword;
        }
        let new_user = NewUser { username: login, secret: password };

        return match diesel::insert_into(users::table).values(new_user).get_result::<User>(&self.0) {
            Ok(user) => {
                let claims = Claims{
                    sub: user.id.to_string().clone(),
                    exp: (chrono::Utc::now()+chrono::Duration::hours(1)).timestamp() as usize
                };
                let token = match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("some_secret".as_ref())
                ) {
                    Ok(tok) => tok,
                    Err(e ) => {
                        eprintln!("Error Generating Token {}",e);
                        return RegistrationOutcome::Other
                    }
                };
                RegistrationOutcome::Ok(json!({"token":token}))},
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => RegistrationOutcome::AlreadyInUse,
            _ => RegistrationOutcome::Other,
        };
    }
}

 // pub async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>,StatusCode>{
 //     if let Some(auth_header) = header_map.get("Authorization"){
 //        if let Ok(auth_header_str) = auth_header.to_str(){
 //            if auth_header_str.starts_with("Bearer"){
 //                let token = auth_header_str.trim_start_matches("Bearer").to_string();
 //                match decode :: <Claims>(&token, &DecodingKey::from_secret( "some_secret".as_ref()),&Validation::default()){
 //                    Ok(_)=> {
 //                        let info: &str = "Valid".to;
 //                        return Ok(Json(info));
 //                }
 //                    Err(e) =>{
 //                        eprintln!("Error Generating Token {}",e);
 //                        return Err(StatusCode::Unauthorized)
 //                    }
 //                }
 //            }
 //        }
 //     }
 //     Err(StatusCode::Unauthorized)
 // }