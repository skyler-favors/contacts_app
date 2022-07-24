use yew::prelude::*;
use std::rc::Rc;
use titlecase::titlecase;
use yew_hooks::prelude::*;
use crate::shared::{Contact, seive};

use super::MessageContext;

#[function_component(List)]
pub fn list() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
    let filters = &msg_ctx.filters;
    let contacts = &msg_ctx.contacts;
    let contacts = seive(contacts.to_vec(), filters);

    let list_items = contacts.iter().map(|contact| {
        html! {
            <li><ListItem {contact} /></li>
        }
    }).collect::<Html>();

    html! {
        <ol>
            {list_items}
        </ol>
    }
}

#[derive(Properties, PartialEq)]
struct ListItemProps {
    contact: Rc<Contact>
}

#[function_component(ListItem)]
fn list_item(props: &ListItemProps) -> Html {
    let contact = &props.contact;
    let toggle_list = use_bool_toggle(false);

    let onclick_list = {
        let toggle = toggle_list.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <>
        <button onclick={onclick_list} class={classes!("text-xl", "font-bold")}>
            { format!("{} {}", titlecase(&contact.firstname), titlecase(&contact.lastname)) }
        </button>

        if *toggle_list {
            <ol>
                <li>{format!("Favorite: {}", &contact.favorite)}</li>
                <li>{format!("Active: {}", &contact.active)}</li>
            </ol>
        }
        </>
    }
}
