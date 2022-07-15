use yew::prelude::*;
use crate::components::{list::ListContacts, create::CreateContact};

mod components;
mod shared;

// App entry point
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class={classes!("flex", "justify-center", "h-full", "flex-col")}>
            <h1 class={classes!("flex", "justify-center", "text-2xl")}>{ "Contacts!" }</h1>
            <CreateContact />
            <ListContacts />
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
