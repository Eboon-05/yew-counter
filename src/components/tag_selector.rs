use std::ops::{Not, Deref};

use yew::prelude::*;

use yew_icons::{Icon, IconId};

use crate::counter_ctx::{CounterContext, structs::CounterAction};

#[function_component(TagSelector)]
pub fn tag_selector() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let active = use_state(|| false);

    let toggle_active = Callback::from({
        let active = active.clone();

        move |_| {
            active.set(active.not())
        }
    });

    let toggle_tag = Callback::from({
        // let selected_tags = counter_ctx.selected_tags.clone();
        let counter_ctx = counter_ctx.clone();

        move |tag: String| {
            let mut new_selected_tags = counter_ctx.selected_tags.clone();
            
            if new_selected_tags.contains(&tag) {
                new_selected_tags.retain(|t| t != &tag)
            } else {
                new_selected_tags.push(tag)
            }

            counter_ctx.dispatch(CounterAction::SelectTags(new_selected_tags))
        }
    });

    html! {
        <div>
            <div class={classes!(
                Some("w-full p-4 bg-white dark:bg-black dark:text-white flex justify-between items-center"),
                active.then_some("rounded-t-xl"),
                active.not().then_some("rounded-xl mb-2"),
            )} onclick={toggle_active}>
                <span class="text-lg">
                    {"Filter by tags"}
                </span>
                <div class="flex justify-end items-center">
                    <button class={classes!(
                        Some("transition-transform"),
                        active.then_some("rotate-180"),
                    )}>
                        <Icon icon_id={IconId::HeroiconsOutlineChevronDown} />
                    </button>
                </div>
            </div>
            if *active.deref() {
                <ul class="w-full p-4 rounded-b-xl bg-white bg-opacity-30 grid grid-cols-2 sm:grid-cols-3 gap-3 mb-2">
                    { counter_ctx.tags.clone().into_iter().map(|tag| {
                        let counter_ctx = counter_ctx.clone();
    
                        html! {
                            <ListTag tag={tag.clone()} toggle_tag={toggle_tag.clone()} checked={counter_ctx.selected_tags.contains(&tag)} />
                        }
                    }).collect::<Html>() }
                </ul>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ListTagProps {
    tag: String,
    toggle_tag: Callback<String>,
    checked: bool,
}

#[function_component(ListTag)]
fn list_tag(props: &ListTagProps) -> Html {
    let tag = props.tag.clone();
    let callback = props.toggle_tag.reform(move |_: MouseEvent| tag.to_owned());

    html! {
        <li class="flex justify-start items-center">
            <input type="checkbox" checked={props.checked} class="mr-2" onclick={callback} />
            {props.tag.clone()}
        </li>
    }
}