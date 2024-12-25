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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FLData {
    pub new_password: String
}
