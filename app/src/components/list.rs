use super::msg_ctx::MessageContext;
use crate::shared::{seive, Contact, toggle_fav, toggle_trash};
use titlecase::titlecase;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_octicons::{IconKind, Icon};
use crate::msg_ctx::Actions;
use crate::form::JsonForm;

#[function_component(List)]
pub fn list() -> Html {
    let ctx = use_context::<MessageContext>().unwrap();
    let filters = &ctx.filters;
    let contacts = &ctx.contacts;
    let contacts = seive(contacts, filters);

    let list_items = contacts
        .iter()
        .map(|contact| {
            html! {
                <li><ListItem contact={contact.clone()} /></li>
            }
        })
        .collect::<Html>();

    html! {
        <ol>
            {list_items}
        </ol>
    }
}

#[derive(Properties, PartialEq)]
struct ListItemProps {
    contact: Contact,
}

#[function_component(ListItem)]
fn list_item(props: &ListItemProps) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
    let contact = &props.contact;
    let toggle_list = use_bool_toggle(false);

    let mut heart_icon = Icon::new_big(IconKind::Heart);
    let mut trash_icon = Icon::new_big(IconKind::Trash);
    if !contact.active {
        trash_icon = Icon::new_big(IconKind::Upload);
    }
    if contact.favorite {
        heart_icon = Icon::new_big(IconKind::HeartFill);
    }

    let onclick_list = {
        let toggle = toggle_list.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let id = contact.id;
    let get_trash = use_async(async move {
        toggle_trash(id).await
    });

    let onclick_trash = {
        let ctx = msg_ctx.clone();
        let contact = contact.clone();
        Callback::from(move |_| {
            ctx.dispatch(Actions::Trash(contact.clone()));
            get_trash.run()
        })
    };

    let id = contact.id;
    let get_fav = use_async(async move {
        toggle_fav(id).await
    });

    let onclick_fav = {
        let ctx = msg_ctx;
        let contact = contact.clone();
        Callback::from(move |_| {
            ctx.dispatch(Actions::Favorite(contact.clone()));
            get_fav.run();
        })
    };

    let toggle_edit = use_bool_toggle(false);

    let onclick_edit = {
        let toggle_edit = toggle_edit.clone();
        Callback::from(move |_| {
            toggle_edit.toggle()
        })
    };

    html! {
        <>
        <div>
            <button onclick={onclick_list} class={classes!("text-xl", "font-bold")}>
                { format!("{} {}", titlecase(&contact.firstname), titlecase(&contact.lastname)) }
            </button>

            <button onclick={onclick_trash} class={classes!("mx-3", "float-right")}>
                <i>{ trash_icon }</i>
            </button>

            <button onclick={onclick_fav} class={classes!("mx-3", "float-right")}>
                <i>{ heart_icon }</i>
            </button>

            <button onclick={onclick_edit} class={classes!("mx-3", "float-right")}>
                <i>{ Icon::new_big(IconKind::Pencil) }</i>
            </button>
        </div>

        if *toggle_edit {
            <div>
                <JsonForm contact={contact.clone()}/>
            </div>
        }

        if *toggle_list {
            <ol>
                <li>{format!("Nickname: {}", &contact.nickname)}</li>
                <li>{format!("Company: {}", &contact.company)}</li>
                <li>{format!("Website: {}", &contact.url)}</li>
                <li>{format!("Notes: {}", &contact.notes)}</li>
                <li>{format!("Favorite: {}", &contact.favorite)}</li>
                <li>{format!("Active: {}", &contact.active)}</li>
                <li>{format!("Street: {}", &contact.street)}</li>
                <li>{format!("City: {}", &contact.city)}</li>
                <li>{format!("State: {}", &contact.state)}</li>
                <li>{format!("Zip-Code: {}", &contact.zip)}</li>
                <li>{format!("Country: {}", &contact.country)}</li>
                <li>{format!("Emails: {:?}", &contact.emails)}</li>
                <li>{format!("Phone Numbers: {:?}", &contact.phone_numbers)}</li>
            </ol>
        }
        </>
    }
}
