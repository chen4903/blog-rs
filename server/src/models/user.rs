use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin, sync::Arc};
use cookie::Cookie;
use ntex::{
    http::HttpMessage,
    web::{ErrorRenderer, FromRequest}
};
use reqwest::Client;
use crate::{errors::CustomError, AppState};

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

/// 网站的所有用户（包括管理员）（用于身份验证）
#[derive(Debug, Clone)]
pub struct User{
    pub id: i32
}

/// 返回给客户端的用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// GitHub用户ID
    pub id: i32,
    /// 用户名（不是昵称）
    pub login: String,
    /// 用户头像的地址
    pub avatar_url: String,
    /// 是否为管理员
    pub is_admin: bool
}

/// 网站的管理员（用于身份验证）
#[derive(Debug, Clone)]
pub struct Admin{
    pub id: i32
}

// 实现FromRequest trait
// 可以从请求中提取用户数据并且验证用户的身份
// async fn handler(user: User / admin: Admin)
// 这样就可以为具体的 handler 添加身份认证了

// 通过验证的用户都是存在于我们的数据库中的，可以通过 ID 查到

impl<E: ErrorRenderer> FromRequest<E> for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        // 注意：下面的两个变量的类型不能出现引用，否则回出现生命周期问题（future）
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        // Cookies中的access token
        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into()))
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!( "SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_none() 
            {
                // 查不到
                // 用户没有在本站使用GitHub登录过
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用GitHub登陆过, 请登录".into()
                ));
            }

            Ok(Self {
                id: user_id
            })
        };

        Box::pin(fut)
    }
}

impl<E: ErrorRenderer> FromRequest<E> for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        // 注意： 下面的两个变量的类型不能出现引用（req），否则就会出现生命周期问题（future）
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();
        
        // Cookies中的access token 
        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into()))
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_some()
            {
                // 查到了
                // 需要管理员权限
                // 管理员的Github ID
                if user_id != 108803001 {
                    // 用户不是管理员
                    return Err(CustomError::AuthFailed(
                        "你不是管理员, 无权执行该搞作".into()
                    ));
                }
            }else {
                // 查不到
                // 用户没有在本站使用GitHub登陆过
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用GitHub登陆过, 请登录".into()
                ));
            }

            Ok(Self {
                id: user_id
            })
        };
        Box::pin(fut)
    }
}

async fn get_user_id(access_token: &Cookie<'_>) -> Result<i32, CustomError> {
    let client = Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        // Github 的API要求我们设置UA
        .header("User-Agent", "blog-rs")
        .send()
        .await;

    let user_id = match user_info {
        Ok(r) => {
            match r.json::<GithubUserInfo>().await {
                Ok(i) => i.id,
                Err(_) => {
                    // 无法解析，可能是GitHub返回了错误消息
                    return Err(CustomError::BadRequest(
                        "无法获取GitHub用户信息, 可能是提供了不正确的access token, 请重新登录".into(),
                    ))
                }
            }
        }
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取GitHub用户信息, 请重试".into()
            ))
        }
    };

    Ok(user_id)
}