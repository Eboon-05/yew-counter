use web_sys::window;
use yew::prelude::*;
use yew_router::components::Link;

use crate::app::Route;

use yew_icons::{Icon, IconId};

struct Path {
    name: &'static str,
    pathname: &'static str,
    path: Route,
    icon: IconId,
}


#[function_component(Navbar)]
pub fn navbar() -> Html {
    let paths: Vec<Path> = vec![
        Path {
            name: "Home",
            pathname: "/",
            path: Route::Home,
            icon: IconId::HeroiconsSolidHome
        },
        Path {
            name: "New count",
            pathname: "/new-count",
            path: Route::NewCount,
            icon: IconId::HeroiconsSolidPlus,
        },
    ];

    let window = window().unwrap();
    let pathname = window.location().pathname().unwrap();

    html! {
        <nav class="fixed inset-x-4 bottom-4">
            <ul class="grid grid-cols-2 items-center justify-evenly justify-items-center">
                {paths.into_iter().map(|p| {
                    let matches = pathname == p.pathname;

                    html! {
                        <li 
                            class={classes!(
                                Some("rounded-full p-3 font-pacifico"),
                                matches.then_some("bg-white dark:bg-black dark:text-white"),
                            )}
                        >
                            <Link<Route> to={p.path} classes={classes!(
                                Some("flex items-center"),
                                matches.then_some("dark:gradient-text")
                            )}>
                                <Icon icon_id={p.icon} class="w-5 h-5 mr-2" />
                                {p.name}
                            </Link<Route>>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </nav>
    }
}