use std::rc::Rc;
use web_sys::{window, console};
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Count {
    pub name: String,
    pub value: i64,
    pub tags: Vec<String>,
}

pub enum CounterAction {
    ToggleTheme,
    Increment(usize, i64),
    Decrement(usize, i64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Store {
    pub dark_theme: bool,
    pub counts: Vec<Count>,
}

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
            },
            CounterAction::Increment(index, amount) => {
                if let Some(_) = counts.get(index) {
                    counts[index].value += amount;
                }
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
    let ls_theme = local_storage.get_item("dark_theme").unwrap();

    let mut dark_theme = true;

    if ls_theme.is_some() {
        let parsed = serde_json::from_str::<bool>(&ls_theme.unwrap());

        if parsed.is_ok() {
            dark_theme = parsed.unwrap();
        }
    } else {
        local_storage.set_item("dark_theme", true.to_string().as_str()).unwrap();
    }

    let store = use_reducer(|| Store {
        dark_theme,
        counts: Vec::from([Count {
            name: "Test count".to_string(),
            value: 0,
            tags: Vec::from(["test".to_string(), "work".to_string()]),
        }]),
    });

    use_effect_with_deps({
        let store = store.clone();
        let local_storage = local_storage.clone();

        move |_| {
            console::log_1(&"theme changed".into());
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