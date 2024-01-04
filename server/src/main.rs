mod errors;
mod article;
mod modles;

use crate::errors::CustomError;
use ntex::web::{self, middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::{Arc, Mutex}};
use article::view;

#[derive(Debug, Clone)]
pub struct AppState{
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() {
    dotenvy::dotenv().ok(); // .env

    // 设置日志等级
    env::set_var("RUST_LOG", "ntex=info"); 
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set `DATABASE_URL`");

    // State
    let app_state = Arc::new(Mutex::new(AppState{
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    }));

    HttpServer::new( move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .service(view::get_articles)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap()
}