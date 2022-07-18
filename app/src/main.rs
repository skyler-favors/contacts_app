use yew::prelude::*;
//use crate::components::{contact_list::ContactList, create_contact::CreateContact};
use crate::components::contact_list::ContactList;

mod components;
mod shared;

// App entry point
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class={classes!("flex", "justify-center", "h-full", "flex-col")}>
            <h1 class={classes!("flex", "justify-center", "text-2xl")}>{ "Contacts!" }</h1>
//            <CreateContact />
            <ContactList />
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
