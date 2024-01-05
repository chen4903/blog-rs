mod errors;
mod article;
mod modles;

use article::{edit, new ,view, delete, search};
use ntex::web::{middleware, App, HttpServer, self};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

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
    let app_state = Arc::new(AppState{
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });

    HttpServer::new( move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .configure(route)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap()
}

fn route(cfg: &mut web::ServiceConfig){
    cfg.service(
            web::scope("/article")
            .route("{id}", web::get().to(
                view::get_article))
            .route("", web::post().to(
                new::new_article
            ))
            .route("", web::put().to(
                edit::edit_article
            ))
            .route("{id}", web::delete().to(
                delete::delete_article
            ))
            .route("/search/{keyword}", web::get().to(
                search::search_article)),
        )
        .service(web::scope("/articles").route("", web::get().to(view::get_articles_preview))
    );
} 