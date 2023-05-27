use yew::prelude::*;

use crate::components::count_list::CountList;
use crate::components::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main class="min-h-screen w-screen bg-transparent">
            <Header />
            <CountList />
        </main>
    }
}
