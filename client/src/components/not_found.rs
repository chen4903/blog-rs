use yew::prelude::*;
use crate::components::{card::Card, container::AppContext};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    // 通过Callback更改网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit("找不到网页".into());

    html! {
        <Card title={"找不到该网页"}>
            <p>{"尝试换个网址?"}</p>
        </Card>
    }
}