use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginActions {
    pub redirect: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
