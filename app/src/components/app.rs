use yew::prelude::*;
use std::rc::Rc;
use crate::components::*;
use crate::shared::Contact;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub contacts: Rc<Vec<Rc<Contact>>>,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let contacts = Rc::clone(&props.contacts);
    let filter: UseListHandle<Filter> = use_list(Vec::new());

    html! {
        <>
        </>
    }
}
