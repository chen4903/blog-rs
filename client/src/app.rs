use crate::components::{
    container::Container,
    home::Home,
    not_found::NotFound,
    article::{view::ArticleViewer, new::NewArticle, delete::DeleteArticle, edit::EditArticle},
    user::{login::Login, oauth::OAuth}
};
use yew::prelude::*;    
use yew_router::prelude::*;


// app 函数是一个Yew组件，使用了 #[function_component] 属性宏。该组件是整个应用的入口点。
#[function_component(App)]
pub fn app() -> Html {
    html! {
        // 在组件的渲染部分，使用了 BrowserRouter 和 Switch 组件，它们是Yew Router库提供的用于处理路由的组件。
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// Route 是一个枚举类型，用于定义应用的主要路由。这里有两个变体：Home 和 NotFound。
// 使用了 Routable 属性宏来指定路由的配置，
// 例如 #[at("/")] 表示 Home 路由匹配根路径，#[at("/404")] 表示 NotFound 路由匹配 "/404" 路径。
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/article/:article_id")]
    ArticleViewer{article_id: u32},
    #[at("/article/new")]
    NewArticle,
    #[at("/article/edit/:article_id")]
    EditArticle { article_id: u32},
    #[at("/article/delete/:article_id")]
    DeleteArticle { article_id: u32},
    #[at("/user/login")]
    Login,
    #[at("/user/login/oauth")] // Github跳转回来的时候，会携带一些参数
    OAuth,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// 路由切换函数
fn switch(route: Route) -> Html { // switch 函数接收一个 Route 枚举作为参数，根据路由的不同返回不同的HTML。
    html! {
        <Container> // 最终将选择的组件包裹在 Container 组件中。
        {
            match route { // 使用 match 表达式根据路由选择要渲染的组件，分别是 Home 或 NotFound。
                Route::Home => html! { <Home/> },
                Route::ArticleViewer{article_id} => html! {
                    <ArticleViewer{article_id}/>
                },
                Route::NewArticle => html! { <NewArticle/> },
                Route::EditArticle{ article_id} => html! {<EditArticle{article_id} />},
                Route::DeleteArticle{ article_id} => html! {<DeleteArticle{article_id} />},
                Route::Login => html! {<Login/>},
                Route::OAuth => html! {<OAuth/>},
                Route::NotFound => html!{ <NotFound/> }
            }
        }
        </Container>
    }
}