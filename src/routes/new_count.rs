use std::ops::{Deref, Not};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{IconId, Icon};

use crate::{components::{header::Header, navbar::Navbar}, counter_ctx::{structs::{Count, CounterAction}, CounterContext}};

#[function_component(NewCount)]
pub fn new_count() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let name_ref = use_node_ref();
    let initial_value_ref = use_node_ref();
    let tag_ref = use_node_ref();
    let tags: UseStateHandle<Vec<String>> = use_state(|| vec![]);

    let onsubmit: Callback<SubmitEvent> = Callback::from({
        let name_ref = name_ref.clone();
        let initial_value_ref = initial_value_ref.clone();
        let tags = tags.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let name = name_ref.cast::<HtmlInputElement>().expect("No HTML input element for count name");
            let initial_value = initial_value_ref.cast::<HtmlInputElement>().expect("No HTML input element for initial value");

            let num = initial_value.value().parse::<i64>();

            if num.is_ok() {
                let count = Count {
                    name: name.value(),
                    value: num.unwrap(),
                    tags: tags.deref().to_vec(),
                };

                counter_ctx.dispatch(CounterAction::AddCount(count));
            }
        }
    });

    let push_tag = Callback::from({
        let tags = tags.clone();
        let tag_ref = tag_ref.clone();

        move |_: MouseEvent| {
            let tag = tag_ref.cast::<HtmlInputElement>().expect("No HTML input element for tag");
                
            if tags.contains(&tag.value()).not() {
                let mut new_tags = tags.deref().clone();
                
                new_tags.push(tag.value());
                tags.set(new_tags)
            }
        }
    });

    let remove_tag = Callback::from({
        let tags = tags.clone();

        move |t: String| {
            let mut new_tags = tags.deref().clone();
            new_tags.retain(|x| x != &t);
            tags.set(new_tags);
        }
    });

    html! {
        <main class="min-h-screen w-screen bg-transparent flex flex-col justify-between">
            <Header />
            <form class="p-3 mx-2 sm:mx-auto sm:w-screen bg-white dark:bg-black dark:text-white rounded-xl shadow-xl max-w-screen-sm" onsubmit={onsubmit}>
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
                <fieldset class="mb-3">
                    <label for="tag" class="block text-lg font-semibold dark:text-white">
                        {"Tags"}
                    </label>
                    <ul class="mb-3 flex justify-start flex-wrap">
                        {tags.iter().map(|t| {                    
                            html! {
                                <ListTag t={t.clone()} remove_tag={remove_tag.clone()} />
                            }
                        }).collect::<Html>()}
                    </ul>
                    <div class="grid grid-cols-4 gap-2">
                        <input ref={tag_ref} name="tag" type="text" class="col-span-3 bg-white bg-opacity-30 border-2 border-gray-300 dark:border-0 outline-none rounded-xl p-2" />
                        <button type="button" onclick={push_tag} class="w-full p-3 rounded-xl bg-gradient dark:text-black font-semibold">
                            <Icon icon_id={IconId::HeroiconsSolidPlus} class="w-5 h-5 mx-auto dark:text-black" />
                        </button>
                    </div>
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

#[derive(Properties, PartialEq)]
struct ListTagProps {
    t: String,
    remove_tag: Callback<String>
}

#[function_component(ListTag)]
fn list_tag(props: &ListTagProps) -> Html {
    let tag = props.t.clone();
    let callback = props.remove_tag.reform(move |_| tag.to_owned());

    html! {
        <li class="flex justify-start items-center p-2 mr-2 mb-2 bg-black text-white dark:bg-white dark:text-black rounded-full" key={props.t.to_owned()}>
            <span class="mr-2">{props.t.clone()}</span>
            <button type="button" class="bg-gray-700 dark:bg-gray-200 p-1 rounded-full" onclick={callback} data-tag={props.t.to_owned()}>
                <Icon icon_id={IconId::HeroiconsOutlineXMark} class="w-5 h-5 dark:text-black" />
            </button>
        </li>
    }
}