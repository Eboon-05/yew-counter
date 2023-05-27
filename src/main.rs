mod app;
mod components;
mod counter_ctx;
mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
