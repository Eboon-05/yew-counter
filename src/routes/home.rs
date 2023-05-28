use yew::prelude::*;

use crate::components::count_list::CountList;
use crate::components::header::Header;
use crate::components::navbar::Navbar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main class="min-h-screen w-screen bg-transparent">
            <Header />
            <CountList />
            <Navbar />
        </main>
    }
}
