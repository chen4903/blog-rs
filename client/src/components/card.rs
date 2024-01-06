use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    // 由String/&str自动转换而来。用AttrValue的原因：
    // Props里面的字段，他们在内部都被实现为引用计数的，用Rc包裹起来，如果用String，其clone成本较高
    // 而AttrValue的成本较低，yew推荐使用
    pub title: AttrValue, 
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <article class="card" style="margin: auto; margin-top: 5%; width: 80%;">
            <header>
                <h3>{&props.title}</h3>
            </header>
            <footer>
                {for props.children.iter()}
            </footer>
        </article>
    }
}