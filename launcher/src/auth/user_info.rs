use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserInfo {
    pub uuid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AuthData {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_info: UserInfo,
}
