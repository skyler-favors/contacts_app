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
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Nickname:"}</li>
                    <li>{&contact.nickname}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Company: "}</li>
                    <li>{&contact.company}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Website: "}</li>
                    <li>{&contact.url}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Notes: "}</li>
                    <li>{&contact.notes}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Street: "}</li>
                    <li>{&contact.street}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"City: "}</li>
                    <li>{&contact.city}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"State: "}</li>
                    <li>{&contact.state}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Zip-Code: "}</li>
                    <li>{&contact.zip}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Country: "}</li>
                    <li>{&contact.country}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Emails: "}</li>
                    <li>{&contact.emails.iter().map(|s| format!("{}, ", s)).collect::<String>()}</li>
                </div>
                <div class={classes!("flex", "flex-row")}>
                    <li class={classes!("font-bold", "mr-3")}>{"Phone Numbers: "}</li>
                    <li>{&contact.phone_numbers.iter().map(|s| format!("{}, ", s)).collect::<String>()}</li>
                </div>
            </ol>
        }
        </>
    }
}
