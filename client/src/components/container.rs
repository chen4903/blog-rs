use yew_router::prelude::use_navigator; // 是 Yew Router 库提供的一个 hook，用于获取用于导航的 Navigator 对象。
use crate::app::Route;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
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

    html! {
        <>
            <nav> // 导航栏（<nav>）
                <a onclick={jump(Route::Home)} class="brand">
                    <span>{"Blog"}</span>
                </a>
            </nav>

            // 一个全局的 ContextProvider，用于传递设置标题的回调函数给子组件。
            <ContextProvider<Callback<String>> context={set_title}>
                {for props.children.iter()}
            </ContextProvider<Callback<String>>>
        </>
    }
}