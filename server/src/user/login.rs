use cookie::{time::Duration, Cookie};
use ntex::{
    http::Response,
    web::{
        types::{Json, State},
        Responder
    }    
};
use reqwest::Client;
use std::sync::Arc;
use crate::{
    errors::CustomError,
    modles::user::{AccessToken, GithubUserInfo, Login},
    AppState
};

const CLIENT_ID: &str = "b12f78cc1f56e49b9f3e";
const CLIENT_SECRET: &str = "a1d15f98d4dccc49102d762ef62cd5de2c06d580";

/// 接收传过来的code，获取access_token，得到用户的数据并存进数据库
pub async fn github_login(
    code: Json<Login>,
    state: State<Arc<AppState>>
) -> Result<impl Responder, CustomError> { // 在ntex中实现Responder这个trait，他都可以回访
    let code = &code.code;

    // HTTP client
    let client = Client::new();

    // 获取access_token
    // 把Access设置为json，让Github的API给我们返回JSON格式的数据
    let access_token = client
        .post(format!(
            "https://github.com/login/oauth/access_token?client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}"
        ))
        .header("Accept", "application/json")
        .send()
        .await;

    let access_token = match access_token {
        // 将数据结构出来，反序列化成结构体AccessToken
        Ok(r) => match r.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                "code是无效的(可能已经过期), 请重新使用GitHub登录".into(),
            ))
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取access_token, 请重试".into()
            ));
        }
    };

    let user_into = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        // 第二个参数是项目的名字，或者填写你的GitHub用户名
        .header("User-Agent", "blog-rs")
        .send()
        .await;

    let user_info = match user_into {
        Ok(r) => r.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取GitHub用户信息,请重试".into()
            ));
        }
    };

    // 设置cookie，这样用户就不需要重复登录了
    let mut cookie = Cookie::new("ACCESS_TOKEN",access_token);
    cookie.set_path("/"); // 跟目录下cookie都有效
    cookie.set_max_age(Duration::days(7)); // 七天内cookie有效
    // 设置：cookie只能通过http协议访问到，也就是说，浏览器在每次请求的时候，都在headers里面附上这么一个cookie；
    // 浏览器本来是可以从js访问到cookie，但是设置了这个，js就无法访问到cookie，提高了安全性
    cookie.set_http_only(true); 

    // 把用户信息存进数据库
    let db_pool = &state.db_pool;

    // 如果已经有一条相同的ID记录，则更新；否则就新增
    sqlx::query!(
        "INSERT INTO users (id, name, avatar_url) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, avatar_url = $3",
        user_info.id,
        user_info.login,
        user_info.avatar_url
    )
    .execute(db_pool)
    .await?;

    let mut response = Response::Ok().body(format!(
        "Hi, {}!", user_info.login
    ));

    // 忽略错误
    let _ = response.add_cookie(&cookie);

    Ok(response)

}