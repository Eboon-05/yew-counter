use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(NewCount)]
pub fn new_count() -> Html {
    html! {
        <main class="min-h-screen w-screen bg-transparent">
            <Navbar />
        </main>
    }
}
