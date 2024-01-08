use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    app::Route, 
    models::article::ArticlePreview as Preview, 
    components::container::AppContext
};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub articles: Result<Vec<Preview>, String>
}

/// 查看所有文章（预览）
#[function_component(ArticlePreview)]
pub fn article_preview(props: &Props) -> Html {
    // 用于跳转到其他路由（其他组件）
    let navigator = use_navigator().unwrap();

    let user = use_context::<AppContext>().unwrap().user;
    let is_admin = if let Ok(user) = &*user {
        user.is_admin
    } else {
        false
    };

    html! {
        { content(navigator, &props.articles, is_admin) }
    }
}

/// 生成HTML
fn content(
    navigator: Navigator, 
    articles: &Result<Vec<Preview>, String>,
    is_admin: bool
) -> Html {
    let jump = |navigator: Navigator, router:Route| {
        Callback::from(move |_| {
            // 查看对应的文章
            navigator.push(&router)
        })
    };

    match articles {
        Ok(articles) => {
            // 数据库里没有文章
            if articles.is_empty() {
                html! {
                    <p> {"似乎没有文章"}</p>
                }
            } else {
                    articles
                .iter()
                .map(|i| {
                    html! {
                        // 因为jump会把navigator移动（move），这样就无法在迭代器中使用了（因为在上次迭代中这个变量已经被move了，所以在接下来的迭代中就无法使用了），所以要clone一下
                        <article class="card" key={i.id}>
                            <header>
                                <div onclick={jump(navigator.clone(),
                                Route::ArticleViewer {article_id: i.id})}>
                                    <h3>{ &i.title}</h3>
                                    <span style="color: grey;">{ &i.date}</span>
                                </div>
                                if is_admin {
                                    <button onclick={jump(navigator.clone(),
                                    Route::EditArticle {article_id: i.id})}>{"修改"}</button>
                                    // 距离左边的按钮1%的距离
                                    <button style="margin-left: 1%;" class="error" onclick={jump(navigator.clone(),
                                    Route::DeleteArticle {article_id: i.id})}>{"删除"}</button>
                                }
                            </header>
                        </article>
                    }
                })
                .collect::<Html>()
            }
        }
        Err(e) => html! {
            <p>{e}</p>
        }
    }
}