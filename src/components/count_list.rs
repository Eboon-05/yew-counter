use yew::prelude::*;

use yew_icons::{Icon, IconId};

use crate::counter_ctx::{CounterAction, CounterContext};

#[function_component(CountList)]
pub fn count_list() -> Html {
    let counter_ctx = use_context::<CounterContext>().unwrap();

    html! {
        <section class="px-3 pt-4">
            {
                counter_ctx.counts.clone().into_iter().map(|c| {
                    let i = counter_ctx.clone().counts.iter().position(|count| count == &c).unwrap();

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
                                {c.name}
                            </div>
                            <div class="text-xl font-bold text-end">
                                <span class="bg-white bg-opacity-40 p-2 rounded-lg">
                                    {c.value}
                                </span>
                            </div>
                            <button class="bg-white shadow rounded-lg w-full py-2" onclick={decrement}>
                                <Icon icon_id={IconId::HeroiconsSolidMinus} class="mx-auto w-5 h-5" />
                            </button>
                            <button class="bg-white shadow rounded-lg w-full py-2" onclick={increment}>
                                <Icon icon_id={IconId::HeroiconsSolidPlus} class="mx-auto w-5 h-5" />
                            </button>
                            <div class="col-span-2 flex items-center text-sm">
                                <Icon icon_id={IconId::FontAwesomeSolidTags} class="mr-2 text-white" />
                                {c.tags.into_iter().map(|t| {
                                    html! {
                                        <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{t}</span>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </section>
    }
}
