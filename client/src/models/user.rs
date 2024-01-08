use serde::{Deserialize, Serialize};

/// 用户信息
/// 注意要实现PartialEq
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    /// Github用户ID
    pub id: u32,
    /// 用户名（不是昵称）
    pub login: String,
    /// 用户头像的地址
    pub avatar_url: String,
    /// 是否为管理员
    pub is_admin: bool
}

/// 用于OAuth登陆时从路径中 提取query参数和向后端发起请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    pub code: String
}