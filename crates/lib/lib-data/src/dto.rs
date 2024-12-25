use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum ServiceStatus {
    Offline,
    Online,
    Disabled,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OtpFormat {
    Numeric = 0,
    Alphanumeric = 1,
    All = 2,
}
