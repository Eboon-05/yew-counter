use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="px-3 py-6 shadow-md rounded-b-3xl flex justify-between items-center bg-white">
            <div class="text-5xl">
                {"Counter"}
            </div>
            <div>
                {"="}
            </div>
        </header>
    }
}