use yew::prelude::*;
use yew_router::components::Link;

use crate::{components::header::Header, app::Route};

#[function_component(NewCount)]
pub fn new_count() -> Html {
    html! {
        <main class="min-h-screen w-screen bg-transparent">
            <Header />
            <Link<Route> to={Route::Home}>
                {"<- Go back to Home"}
            </Link<Route>>
        </main>
    }
}
