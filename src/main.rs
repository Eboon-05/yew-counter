mod app;
mod components;
mod counter_ctx;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
