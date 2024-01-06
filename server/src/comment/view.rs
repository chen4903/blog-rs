use std::sync::Arc;
use ntex::web::types::{Json, Path, State};
use crate::{
    errors::CustomError,
    models::{comment::Comment, user::GithubUserInfo}, AppState
};
/// 通过文章ID获取该文章的所有评论（包含发表评论的用户的信息）
pub async fn get_comments_for_article(
    article_id: Path<(u32,)>,
    state: State<Arc<AppState>>
) -> Result<Json<Vec<Comment>>, CustomError> {
    let db_pool = &state.db_pool;
    let article_id = article_id.0;

    // 查找对应问责的所有评论，拿到他们的user_id, content, date和users表里相同user_id(对应的是users表里的id)的记录的name, avatar_url
    let comments = sqlx::query!(
        "SELECT comments.user_id, comments.content, comments.date, users.name, users.avatar_url FROM comments JOIN users ON comments.user_id = users.id WHERE comments.article = $1", article_id as i32
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| Comment {
        id: None,
        user: Some(GithubUserInfo {
            id: i.user_id,
            login: i.name.clone(),
            avatar_url: i.avatar_url.clone()
        }),
        content: i.content.clone(),
        date: Some(i.date.unwrap()),
        article: None
    })
    .collect::<Vec<Comment>>();

    Ok(Json(comments))
}