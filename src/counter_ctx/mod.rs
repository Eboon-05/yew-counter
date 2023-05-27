use std::rc::Rc;
use web_sys::{window};
use yew::prelude::*;

mod utils;
pub mod structs;

use utils::*;
use structs::*;

impl Reducible for Store {
    type Action = CounterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let dark_theme = match action {
            CounterAction::ToggleTheme => !self.dark_theme,
            _ => self.dark_theme,
        };

        let mut counts = self.counts.clone();

        match action {
            CounterAction::Decrement(index, amount) => {
                if let Some(_) = counts.get(index) {
                    counts[index].value -= amount;
                }

                save_counts(&counts)
            },
            CounterAction::Increment(index, amount) => {
                if let Some(_) = counts.get(index) {
                    counts[index].value += amount;
                }

                save_counts(&counts)
            },
            _ => ()
        }

        Store {
            dark_theme,
            counts,
        }
        .into()
    }
}

pub type CounterContext = UseReducerHandle<Store>;

#[derive(Properties, Debug, PartialEq)]
pub struct CounterProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CounterProvider)]
pub fn counter_provider(props: &CounterProviderProps) -> Html {
    let window = window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    let store = use_reducer(|| Store {
        dark_theme: get_dark_theme(),
        counts: get_counts(),
    });

    use_effect_with_deps({
        let store = store.clone();
        let local_storage = local_storage.clone();

        move |_| {
            let document = window.clone().document().unwrap();
            let html = document.query_selector("html").unwrap().unwrap();
    
            local_storage.set_item("dark_theme", store.dark_theme.to_string().as_str()).unwrap();

            if store.dark_theme {
                html.set_class_name("dark");
            } else {
                html.set_class_name("");
            }
        }
    }, store.dark_theme);

    html! {
        <ContextProvider<CounterContext> context={store}>
            {props.children.clone()}
        </ContextProvider<CounterContext>>
    }
}