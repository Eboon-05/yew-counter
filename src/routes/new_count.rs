use yew::prelude::*;

use crate::{components::{header::Header, navbar::Navbar}};

#[function_component(NewCount)]
pub fn new_count() -> Html {
    html! {
        <main class="min-h-screen w-screen bg-transparent">
            <Header />
            <form>
                <label>
                    {"Count name"}
                    <input type="text" required={true} />
                </label>
            </form>
            <Navbar />
        </main>
    }
}
