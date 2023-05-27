use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{
    home::Home,
    new_count::NewCount,
};

use crate::counter_ctx::CounterProvider;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/new-count")]
    NewCount,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NewCount => html! { <NewCount /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CounterProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            <div class="fixed inset-0 -z-[1] hidden dark:block bg-black bg-opacity-30"></div>
            <div class="fixed inset-0 -z-[2] bg-gradient"></div>
        </CounterProvider>
    }
}
