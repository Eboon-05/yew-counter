use yew::prelude::*;

use crate::counter_ctx::{CounterContext, CounterAction};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let toggle_theme = Callback::from(move |_: MouseEvent| {
        counter_ctx.dispatch(CounterAction::ToggleTheme)
    });

    let counter_ctx = use_context::<CounterContext>().unwrap();

    html! {
        <header class="px-3 py-6 shadow-md rounded-b-3xl flex justify-between items-center bg-white">
            <div class="text-5xl">
                {"Counter"}
                {counter_ctx.clone().dark_theme}
            </div>
            <button onclick={toggle_theme}>
                {"="}
            </button>
        </header>
    }
}