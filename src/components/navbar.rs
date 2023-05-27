use std::ops::Not;

use yew::prelude::*;

use yew_icons::{Icon, IconId};

use crate::counter_ctx::{CounterAction, CounterContext};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let active = use_state(|| true);
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let toggle_active = {
        let active = active.clone();
        Callback::from(move |_: MouseEvent| active.set(!*active))
    };

    let toggle_dark_mode = {
        let counter_ctx = use_context::<CounterContext>().unwrap();

        Callback::from(move |_: MouseEvent| counter_ctx.dispatch(CounterAction::ToggleTheme))
    };

    html! {
        <header class={classes!(
            Some("px-3 py-6 rounded-b-3xl flex justify-between items-center"),
            active.then_some("bg-transparent"),
            active.not().then_some("bg-white dark:bg-black dark:text-white shadow-md"),
        )}>
            <div class={classes!(
                Some("text-5xl font-bold"),
                active.then_some("")
            )}>
                {"Counter"}
            </div>
            <button onclick={toggle_active}>
                if *active.clone() {
                    <Icon icon_id={IconId::HeroiconsOutlineXMark} class="h-10 w-10" />
                } else {
                    <Icon icon_id={IconId::HeroiconsSolidBars3} class="h-10 w-10" />
                }
            </button>
            <section class={classes!(
                    Some("bg-white dark:bg-black dark:text-white fixed z-10 inset-0 top-[96px] rounded-t-3xl shadow-md px-3 py-6"),
                    active.not().then_some("hidden"),
            )}>
                <h1 class="text-2xl font-bold">{"Settings"}</h1>
                <ul class="max-w-md mx-auto">
                    <li class="flex justify-between items-center w-full">
                        <span>{"Dark mode"}</span>
                        <input value={counter_ctx.dark_theme.to_string()} type="checkbox" onclick={toggle_dark_mode} />
                    </li>
                </ul>
            </section>
        </header>
    }
}
