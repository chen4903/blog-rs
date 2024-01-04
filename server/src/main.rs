mod errors;

use crate::errors::CustomError;

use ntex::web::{self, middleware, App, HttpServer};
use std::env;

#[ntex::main]
async fn main() {
    // 设置日志等级
    env::set_var("RUST_LOG", "ntex=info"); 
    env_logger::init();

    HttpServer::new( || {
        App::new()
            .wrap(middleware::Logger::default()) // 添加中间件：日志服务
            .service(index)
            .service(error)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap()
}

#[web::get("/")]
async fn index() -> String { // 需要实现Responder这个 trait
    "Hello, world".into()
}

#[web::get("/error")]
async fn error() -> Result<String, CustomError> {
    Err(CustomError::NotFound("Not found".into()))
}