use yew::prelude::*;

use crate::components::count_list::CountList;
use crate::components::navbar::Navbar;
use crate::counter_ctx::CounterProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CounterProvider>
            <main class="min-h-screen w-screen">
                <Navbar />
                <CountList />
            </main>
        </CounterProvider>
    }
}
