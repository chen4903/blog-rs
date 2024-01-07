use yew::prelude::*;
use crate::components::{card::Card, container::AppContext};

#[function_component(Login)]
pub fn login() -> Html {
    // 设置网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("登录".into());

    html! {
        <Card title={"登录"}>
            <a class="button mainButton" href="https://github.com/login/oauth/authorize?client_id=b12f78cc1f56e49b9f3e">{"使用Github登录"}</a>
        </Card>
    }
}