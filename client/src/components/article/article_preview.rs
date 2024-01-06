use gloo::net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::{app::Route, fetch::fetch, models::article::ArticlePreview as Preview};

#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    // 表示数据正在加载中，在页面上显示Loading字样
    let loading = use_state(|| true);
    let articles = use_state(|| Err("".into()));

    // 用于跳转到其他路由（其他组件）, 这是一个钩子Hook
    let navigator = use_navigator().unwrap();

    {
        let loading = loading.clone();
        let articles = articles.clone();
        use_effect_with_deps(
            move |_| { // 这里move进去了，但是我们后面还要用loading, articles，因此我们加上{}作用域，后面还可以再用loading, articles
                wasm_bindgen_futures::spawn_local(async move {
                    articles.set(
                        fetch::<Vec<Preview>>("/api/articles".into(),
                        Method::GET, None).await,
                    );
                    loading.set(false); // 成功获取数据之后，Loading设置为false
                });
            }, 
            // dependencies是一个()，说明在组件执行成功的时候，只执行一次，无论这个组件以后如何变化，他都不会再次执行了
            // 也就是说，网页加载成功之后，执行唯一的一次
            (), 
        );
    }

    html! {
        if *loading{
            <p>{"Loading..."}</p>
        } else {
            { content(navigator, (*articles).clone())}
        }
    }
}

/// 生成HTML
fn content(
    navigator: Navigator, 
    articles: Result<Vec<Preview>, String>
) -> Html {
    let jump = |navigator: Navigator, article_id: u32| {
        Callback::from(move |_| {
            // 查看对应的文章
            navigator.push(&Route::ArticleViewer{ article_id})
        })
    };

    match articles {
        Ok(articles) => articles
            .iter()
            .map(|i| {
                html! {
                    // 因为jump会把navigator移动(move),这样就无法在迭代器中使用了(因为在上一次迭代中这个变量已经被move了，所以在接下来的迭代中就无法继续使用了)，所以要clone一下
                    <article class="card" onclick={jump(navigator.clone(), i.id)} key={"i.id"}>
                        <header>
                            <h3>{&i.title}</h3>
                            <span style="color: grey">{ &i.date}</span>
                        </header>
                    </article>
                }
            })
            .collect::<Html>(),
        Err(e) => html! {
            <p>{ e }</p>
        }
    }
}