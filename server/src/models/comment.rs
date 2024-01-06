use serde::{Deserialize, Serialize};
use super::user::GithubUserInfo;

/// 评论
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    // 评论ID
    pub id: Option<u32>,
    /// 发表评论的用户的信息
    pub user: Option<GithubUserInfo>,
    /// 评论内容
    pub content: String,
    /// 评论日期
    pub date: Option<chrono::NaiveDate>,
    /// 评论的文章
    pub article: Option<u32>
}