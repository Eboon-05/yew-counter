use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::{header::Header, navbar::Navbar}, counter_ctx::{structs::{Count, CounterAction}, CounterContext}};

#[function_component(NewCount)]
pub fn new_count() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let name_ref = use_node_ref();
    let initial_value_ref = use_node_ref();

    let onsubmit: Callback<SubmitEvent> = Callback::from({
        let name_ref = name_ref.clone();
        let initial_value_ref = initial_value_ref.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let name = name_ref.cast::<HtmlInputElement>().expect("No HTML input element for count name");
            let initial_value = initial_value_ref.cast::<HtmlInputElement>().expect("No HTML input element for initial value");

            let num = initial_value.value().parse::<i64>();

            if num.is_ok() {
                let count = Count {
                    name: name.value(),
                    value: num.unwrap(),
                    tags: vec![],
                };

                counter_ctx.dispatch(CounterAction::AddCount(count));
            }
        }
    });

    html! {
        <main class="min-h-screen w-screen bg-transparent flex flex-col justify-between">
            <Header />
            <form class="p-3 mx-2 bg-white dark:bg-black dark:text-white rounded-xl shadow-xl" onsubmit={onsubmit}>
                <fieldset class="mb-3">
                    <label for="count_name" class="block text-lg font-semibold dark:text-white">
                        {"Count name"}
                    </label>
                    <input ref={name_ref} name="count_name" type="text" required={true} class="bg-white bg-opacity-30 border-2 border-gray-300 dark:border-0 outline-none w-full rounded-xl p-2" />
                </fieldset>
                <fieldset class="mb-3">
                    <label for="initial_value" class="block text-lg font-semibold dark:text-white">
                        {"Initial value"}
                    </label>
                    <input ref={initial_value_ref} name="initial_value" type="number" required={true} class="bg-white bg-opacity-30 border-2 border-gray-300 dark:border-0 outline-none w-full rounded-xl p-2" />
                </fieldset>
                <button type="submit" class="w-full p-3 rounded-xl bg-gradient dark:text-black font-semibold">
                    {"Create"}
                </button>
            </form>
            <div></div>
            <Navbar />
        </main>
    }
}
