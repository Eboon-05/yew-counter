use yew::prelude::*;

use crate::components::count_list::CountList;
use crate::components::navbar::Navbar;
use crate::counter_ctx::CounterProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CounterProvider>
            <main class="min-h-screen w-screen bg-transparent">
                <Navbar />
                <CountList />
                <div class="fixed inset-0 -z-[1] hidden dark:block bg-black bg-opacity-30"></div>
                <div class="fixed inset-0 -z-[2] bg-gradient"></div>
            </main>
        </CounterProvider>
    }
}
