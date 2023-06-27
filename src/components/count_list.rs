use yew::prelude::*;

use yew_icons::{Icon, IconId};

use crate::counter_ctx::structs::Count;
use crate::counter_ctx::{structs::CounterAction, CounterContext};
use crate::components::tag_selector::TagSelector;

#[function_component(CountList)]
pub fn count_list() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    let filtered = use_state(|| counter_ctx.counts.clone());

    use_effect_with_deps({
        let filtered = filtered.clone();
        let counter_ctx = counter_ctx.clone();

        move |_| {
            if counter_ctx.selected_tags.len() > 0 {
                let mut new_filtered: Vec<Count> = vec![];
        
                for count in counter_ctx.counts.iter()  {
                    for tag in count.tags.iter() {
                        if counter_ctx.selected_tags.contains(tag) {
                            let count = count.clone();
                            new_filtered.push(count);
                        }
                    }
                }
        
                filtered.set(new_filtered);
            } else {
                filtered.set(counter_ctx.counts.clone());
            }
        }
    }, counter_ctx.selected_tags.clone());

    html! {
        <section class="px-3 pt-4">
            <TagSelector />
            {
                filtered.iter().map(|c| {
                    let i = counter_ctx.clone().counts.iter().position(|count| count == c).unwrap();

                    let increment = {
                        let counter_ctx = counter_ctx.clone();

                        Callback::from(move |_: MouseEvent| {
                            counter_ctx.dispatch(CounterAction::Increment(i, 1))
                        })
                    };

                    let decrement = {
                        let counter_ctx = counter_ctx.clone();

                        Callback::from(move |_: MouseEvent| {
                            counter_ctx.dispatch(CounterAction::Decrement(i, 1))
                        })
                    };

                    html! {
                        <div class="w-full p-4 rounded-xl bg-white bg-opacity-30 grid grid-cols-2 gap-3 mb-2">
                            <div class="text-lg">
                                {c.name.clone()}
                            </div>
                            <div class="text-xl font-bold text-end">
                                <span class="bg-white bg-opacity-40 p-2 rounded-lg">
                                    {c.value}
                                </span>
                            </div>
                            <button class="bg-white dark:bg-black dark:text-white shadow rounded-lg w-full py-2" onclick={decrement}>
                                <Icon icon_id={IconId::HeroiconsSolidMinus} class="mx-auto w-5 h-5" />
                            </button>
                            <button class="bg-white dark:bg-black dark:text-white shadow rounded-lg w-full py-2" onclick={increment}>
                                <Icon icon_id={IconId::HeroiconsSolidPlus} class="mx-auto w-5 h-5" />
                            </button>
                            if c.tags.len() > 0 {
                                <div class="col-span-2 flex items-center text-sm">
                                    <Icon icon_id={IconId::FontAwesomeSolidTags} class="mr-2 text-white" />
                                    {c.tags.clone().into_iter().map(|t| {
                                        html! {
                                            <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{t}</span>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        </div>
                    }
                }).collect::<Html>()
            }
        </section>
    }
}
