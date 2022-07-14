use yew::prelude::*;
use crate::components::{list::ListContacts, create::CreateContact};

mod components;
mod shared;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Contacts!" }</h1>
            <CreateContact />
            <ListContacts />
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
