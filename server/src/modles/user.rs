use serde::{Deserialize, Serialize};

/// 前端GitHub授权登录后传上来的code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

/// GitHub返回的access_token
#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken{
    pub access_token: String
}

/// GitHub返回的用户信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubUserInfo {
    /// GitHub用户ID
    pub id: i32,
    /// 用户名（不是昵称）
    pub login: String,
    /// 用户头像的地址
    pub avatar_url: String,
}