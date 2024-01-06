use yew::prelude::*;
use crate::components::{article::articles::ArticlePreview, card::Card};

#[function_component(Home)]
pub fn home() -> Html {
    // 通过Callback更改网页标题
    // use_context 是 Yew 框架提供的一个钩子（hook），用于获取上下文中的数据。
    // 在这里，使用 use_context::<Callback<String>>() 获取了一个 Callback<String> 类型的上下文。
    use_context::<Callback<String>>()
        .unwrap()
        // 通过 unwrap() 获取 Callback 中的值，然后使用 .emit("Home".into()) 调用回调函数，向回调函数传递一个标题为 "Home" 的字符串。
        .emit("Home".into());

/* 
    为什么可以修改到网页标题？
    Callback<String> 中的回调函数被提供给了上下文，并且通过 use_context 获取到了这个回调函数。
    在 Yew 框架中，通过调用回调函数，可以触发特定的行为。在这里，回调函数的目的是修改网页标题。
    在回调函数内部，使用了 web_sys::window().unwrap().document().unwrap().set_title 来获取浏览器窗口对象，然后设置文档的标题，从而修改了网页标题。
    总体而言，这段代码中的回调函数充当了修改网页标题的途径，而通过上下文机制，可以在组件中获取这个回调函数并在需要时调用它
*/

    html! {
        <Card title={"文章"}>
            <ArticlePreview/>
        </Card>
    }
}