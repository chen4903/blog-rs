use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
    #[prop_or(html! {})]
    pub footer: Html,
    /// 是否展示这个模态框
    #[prop_or(true)]
    pub open: bool
}

/// 一个模态的弹窗
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    html! {
        <div class="modal">
            // 如果props.open为true，则展示这个框框，否则不展示
            <input id="modal_1" type="checkbox" checked={props.open}/>
            <label for="modal_1" class="overlay"></label>
            <article>
                <header>
                    <h3>{ &props.title }</h3>
                    // 关闭按钮是一个X
                    <label for="modal_id" class="close">{ "✖" }</label>
                </header>
                <section class="children">
                { for props.children.iter()}
                </section>
                <footer>
                { props.footer.clone()}
                </footer>
            </article>
        </div>
    }
}