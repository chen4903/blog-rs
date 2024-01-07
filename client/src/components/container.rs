use gloo::net::http::Method;
use yew_router::prelude::use_navigator; // 是 Yew Router 库提供的一个 hook，用于获取用于导航的 Navigator 对象。
use crate::{app::Route, fetch, models::user::User};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// 应用程序的Context
#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    /// 设置网页的标题
    pub set_title: Callback<String>,
    /// 用户信息（是一个State，因为我们可能要修改里面的数据，并且修改后要更新显示的数据）
    pub user: UseStateHandle<Result<User, String>>
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    // 用于跳转到不同的路由
    // 使用 use_navigator 获取了一个用于导航的 Navigator 对象，并创建了一个用于设置网页标题的回调函数 set_title。
    let navigator = use_navigator().unwrap(); // 他是一个Hook
    let set_title = Callback::from(move |content: String| {
        // 设置网页的标题
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("{content} - Blog"));
    });

    // 创建了一个用于触发路由跳转的回调函数 jump。
    let jump = move |route| Callback::from(
        move |_| navigator.push(&route)
    );

    // 获取用户数据，并放在Context里以便使用
    let user = use_state(|| Err("".into()));

    {
        let user = user.clone();
        // 组件在挂载成功时获取用户数据
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    user.set(
                        fetch::fetch::<User>("/api/user/info".into(), Method::GET, None, None).await
                    )
                })
            }, 
            ()
        );
    }

    // 应用程序的Context
    let context = AppContext{set_title, user};

    html! {
        <>
            <nav> // 导航栏（<nav>）
                <a onclick={
                    // 需要clone一下，以便我们下面多次调用这个闭包
                    let jump = jump.clone();
                    jump(Route::Home)
                } class="brand">
                    <span>{"Blog"}</span>
                </a>
                <div class="menu">
                    if let Ok(user) = (*context.user).clone() {
                        // title: 鼠标悬停有提示
                        // border-radius: 50%：方的图片裁剪成圆的
                        <img src={user.avatar_url} title={format!("Hi, {}!", user.login)} style="width: 7%; border-radius: 50%; float: right;" />
                    } else {
                        // 用户没有登录或者获取用户信息失败
                        <button class="success icon-puzzle" onclick={jump(Route::Login)}>{"登录"}</button>
                    }
                </div>
            </nav>

            // 一个全局的 ContextProvider，用于传递设置标题的回调函数给子组件。
            <ContextProvider<AppContext> context={context}>
                {for props.children.iter()}
            </ContextProvider<AppContext>>
        </>
    }
}