mod app;
mod components;
mod fetch;
mod models;
mod utils;
mod constants;

fn main() {
    yew::Renderer::<app::App>::new().render();
}