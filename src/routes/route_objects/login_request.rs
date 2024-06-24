 use serde::Deserialize;
//use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest<'a> {
    #[serde(rename = "email")]
    pub login: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}
