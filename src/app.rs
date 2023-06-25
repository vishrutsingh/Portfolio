use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{home::Home, about::About, contact::Contact};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch (routes: Route) -> Html {
    match routes {
        Route::Home => html!{<Home />},
        Route::About => html!{<About />},
        Route::Contact => html!{<Contact />},
        Route::NotFound => html!{<h1>{ "401" }</h1>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
