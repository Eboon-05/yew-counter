use std::ops::Not;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    pub checked: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    html! {
        <div 
            class={classes!(
                Some("p-1 w-14 rounded-full"),
                props.checked.then_some("bg-gradient"),
                props.checked.not().then_some("bg-slate-300 dark:bg-slate-800"),
            )}
            onclick={props.onclick.clone()}
        >
            <div class={classes!(
                Some("h-6 w-6 rounded-full bg-white transition-transform"),
                props.checked.then_some("translate-x-full"),
            )}></div>
        </div>
    }
}