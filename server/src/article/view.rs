use ntex::web::{
    self,
    types::{Json, State},
};
use std::sync::{Arc, Mutex};
use crate::{errors::CustomError, modles::article::Article, AppState};

#[web::get("/articles")]
pub async fn get_all_articles(state: State<Arc<AppState>>) -> Result<Json<Vec<Article>>, CustomError>{
    let db_pool = &state.db_pool;

    let articles = sqlx::query!("SELECT * FROM articles")
        .fetch_all(db_pool)
        // 实现了impl From<sqlx::Error> for CustomError ，因此可以使用.await?转换为自定义类型CustomError
        .await? 
        .iter() // 迭代器
        .map(|i| Article {
            id: Some(i.id as u32),
            title: i.title.clone(),
            content: i.content.clone(),
            date: Some(i.date.unwrap()),
        })
        .collect::<Vec<Article>>();

    Ok(Json(articles))
}