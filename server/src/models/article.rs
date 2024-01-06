use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 文章的详细信息，和数据库中表的字段一一对应
pub struct Article {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
}

/// 文章预览
#[derive(Debug, Clone, Serialize)]
pub struct ArticlePreview {
    pub id: u32,
    pub title: String,
    pub date: chrono::NaiveDate,
}