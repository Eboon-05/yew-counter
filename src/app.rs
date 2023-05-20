use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="h-screen">
            <Navbar />
            <section class="px-3 pt-4">
                <div class="w-full p-4 rounded-xl bg-white bg-opacity-30 grid grid-cols-2 gap-3">
                    <div class="text-lg">
                        {"Hello, world!"}
                    </div>
                    <div class="text-xl font-bold text-end">
                        <span class="bg-white bg-opacity-40 p-2 rounded-lg">
                            {"10"}

                        </span>
                    </div>
                    <button class="bg-white shadow rounded-lg w-full py-1">{"+"}</button>
                    <button class="bg-white shadow rounded-lg w-full py-1">{"-"}</button>
                    <div class="col-span-2 flex items-center text-sm">
                        <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{"label"}</span>
                        <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{"label"}</span>
                        <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{"label"}</span>
                        <span class="bg-white bg-opacity-40 p-1 rounded-lg mr-2">{"label"}</span>
                    </div>
                </div>
            </section>
        </main>
    }
}
