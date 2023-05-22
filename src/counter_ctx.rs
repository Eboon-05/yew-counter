use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Count {
    pub name: String,
    pub value: i64,
    pub tags: Vec<String>,
}

pub enum CounterAction {
    ToggleTheme,
    ThemeDark,
    ThemeLight,
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
            CounterAction::ThemeDark => false,
            CounterAction::ThemeLight => true,
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
    let store = use_reducer(|| Store {
        dark_theme: false,
        counts: Vec::from([Count {
            name: "Test count".to_string(),
            value: 0,
            tags: Vec::from(["test".to_string(), "work".to_string()]),
        }]),
    });

    html! {
        <ContextProvider<CounterContext> context={store}>
            {props.children.clone()}
        </ContextProvider<CounterContext>>
    }
}
