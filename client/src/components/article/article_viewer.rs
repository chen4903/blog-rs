use gloo::net::http::Method;
use yew::prelude::*;
use crate::{
    components::{card::Card, container::AppContext},
    models::article::Article,
    fetch,
    utils,
};

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub article_id: u32,
}

/// 查看单篇文章，支持Markdown
#[function_component(ArticleViewer)]
pub fn article_viewer(props: &Props) -> Html {
    let loading = use_state(|| true);
    let article = use_state(|| Err("".into()));

    // 这个变量的声明得放在 use_effect_with_deps外面，否则会遇到声明周期问题(闭包和future: 'static)
    let article_id = props.article_id;

    {
        let loading = loading.clone();
        let article = article.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    article.set(
                        fetch::fetch::<Article>(
                            format!("/api/article/{article_id}"), 
                            Method::GET,
                            None,
                            None, 
                        )
                        .await
                    );
                    loading.set(false)
                });
            }, 
            ()
        );
    }

    let title = if let Ok(article) = (*article).clone() {
        article.title
    } else {
        "文章".into()
    };

    // 设置网页标题
    use_context::<AppContext>()
        .unwrap()
        .set_title
        .emit(title.clone());

    html! {
        if *loading {
            <Card title={"Loading..."}>
                <p>{"马上就好......"}</p>
            </Card>
        } else {
            <Card {title}> // yew提供的语法糖，相当于<Card title="title">
                {
                    match &*article {
                        Ok(article) => utils::convert_markdown_to_html(article.content.clone()),
                        Err(e) => html! {<p>{e} </p>}
                    }
                }
            </Card>
        }
    }
}