use std::sync::Arc;
use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};
use crate::{errors::CustomError, modles::article::Article, AppState};


#[web::put("/article")]
pub async fn edit_article(
    article: Json<Article>,
    state: State<Arc<AppState>>
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    let id = match article.id {
        Some(id) => id,
        None => return Err(CustomError::BadRequest("请提供要修改的文章ID".into())),
    };

    sqlx::query!(
        "UPDATE articles SET title = $1, content = $2 WHERE id = $3",
        article.title,
        article.content,
        id as i32,
    )
    .execute(db_pool)
    .await?;

    Ok("修改文章成功！".into())
}









