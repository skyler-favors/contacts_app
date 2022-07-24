// Components for the contact struct
use titlecase::titlecase;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_octicons::{Icon, IconKind};

use super::Edit;
use crate::shared::*;

#[derive(PartialEq, Clone)]
pub enum ContactListType {
    Normal,
    Trash,
    Favorites,
}

#[derive(Properties, PartialEq)]
pub struct CreateContactListProps {
    pub contacts: Vec<Contact>,
    pub list_type: ContactListType,
    pub state: UseAsyncHandle<Vec<Contact>, Error>,
}

// creates a list item for each contact in vector
#[function_component(CreateContactList)]
pub fn create_contact_list(props: &CreateContactListProps) -> Html {
    // sort contacts by first name
    let mut contacts: Vec<Contact> = props.contacts.clone();
    contacts.sort_by_key(|c| c.firstname.clone());

    html! {
        contacts.iter().map(|contact| {
            html!{
                    <ContactLink list_type={props.list_type.clone()} contact={contact.clone()} state={props.state.clone()} />
            }
        }).collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
struct ContactLinkProps {
    contact: Contact,
    list_type: ContactListType,
    state: UseAsyncHandle<Vec<Contact>, Error>,
}

// make this a struct component so that you can reset the toggle on search
// Makes each list item a button attached to a hidden div with the info
#[function_component(ContactLink)]
fn contact_link(props: &ContactLinkProps) -> Html {
    let contact = &props.contact;
    let id = contact.id;

    let mut heart_icon = Icon::new_big(IconKind::Heart);
    let mut trash_icon = Icon::new_big(IconKind::Trash);
    match props.list_type {
        ContactListType::Normal => {
            if !contact.active {
                return html! {};
            }
            if contact.favorite {
                heart_icon = Icon::new_big(IconKind::HeartFill);
            }
        }
        ContactListType::Trash => {
            if contact.active {
                return html! {};
            }
            trash_icon = Icon::new_big(IconKind::Upload);
            if contact.favorite {
                heart_icon = Icon::new_big(IconKind::HeartFill);
            } else {
                heart_icon = Icon::new_big(IconKind::Heart);
            }
        }
        ContactListType::Favorites => {
            if !contact.favorite || !contact.active {
                return html! {};
            }
            heart_icon = Icon::new_big(IconKind::HeartFill);
            trash_icon = Icon::new_big(IconKind::Trash);
        }
    }

    let toggle_list = use_bool_toggle(false);

    let onclick_list = {
        let toggle = toggle_list.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let toggle_edit = use_bool_toggle(false);

    let onclick_edit = {
        let toggle = toggle_edit.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let mut lastname = "".to_string();
    match &contact.lastname {
        Some(last) => lastname = titlecase(last),
        None => {}
    }

    let reload = props.state.clone();
    let trash_state = use_async(async move {
        let result = toggle_trash(id).await;
        reload.run();
        match result {
            Ok(_x) => Ok(()),
            Err(e) => Err(e),
        }
    });

    let trash_onclick = {
        Callback::from(move |_| {
            trash_state.run();
        })
    };

    let reload = props.state.clone();
    let fav_state = use_async(async move {
        let result = toggle_fav(id).await;
        reload.run();
        match result {
            Ok(_x) => Ok(()),
            Err(e) => Err(e),
        }
    });

    let fav_onclick = {
        Callback::from(move |_| {
            fav_state.run();
        })
    };

    html! {
        <li class={classes!("my-1")}>
        <div>
            <button onclick={onclick_list} class={classes!("text-xl", "font-bold")}>
                { format!("{} {}",titlecase(&contact.firstname), lastname) }
            </button>

            <button onclick={trash_onclick} class={classes!("mx-3", "float-right")}>
                <i>{ trash_icon }</i>
            </button>

            <button onclick={fav_onclick} class={classes!("mx-3", "float-right")}>
                <i>{ heart_icon }</i>
            </button>

            <button onclick={onclick_edit} class={classes!("mx-3", "float-right")}>
                <i>{ Icon::new_big(IconKind::Pencil) }</i>
            </button>
        </div>

        if *toggle_edit {
            <div>
                <Edit {id} contact={contact.clone()} />
            </div>
        }

        if *toggle_list {
            <div class={classes!("border-solid", "border-y-2")}>
                <ul class={classes!("text-zinc-400")}>
                    if let Some(nick) = &contact.nickname {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Nickname: "}</b>
                            <p>{nick}</p>
                        </li>
                    }

                    if let Some(company) = &contact.company {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Company: "}</b>
                            <p>{company}</p>
                        </li>
                    }

                    if let Some(url) = &contact.url {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Website: "}</b>
                            <p>{url}</p>
                        </li>
                    }

                    if let Some(notes) = &contact.notes {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Notes: "}</b>
                            <p>{notes}</p>
                        </li>
                    }

                    <li class={classes!("flex", "flex-row")}>
                        <b class={classes!("mr-5")}>{"Favorite: "}</b>
                        <p>{&contact.favorite}</p>
                    </li>

                    <li class={classes!("flex", "flex-row")}>
                        <b class={classes!("mr-5")}>{"Active: "}</b>
                        <p>{&contact.active}</p>
                    </li>

                    if let Some(street) = &contact.street {
                         <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Street: "}</b>
                            <p>{street}</p>
                        </li>
                    }

                    if let Some(city) = &contact.city {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"City: "}</b>
                            <p>{city}</p>
                        </li>
                    }

                    if let Some(state) = &contact.state {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"State: "}</b>
                            <p>{state}</p>
                        </li>
                    }

                    if let Some(zip) = &contact.zip {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Zip-Code: "}</b>
                            <p>{zip}</p>
                        </li>
                    }

                    if let Some(country) = &contact.country {
                        <li class={classes!("flex", "flex-row")}>
                            <b class={classes!("mr-5")}>{"Country: "}</b>
                            <p>{country}</p>
                        </li>
                    }
                    <li class={classes!("flex", "flex-row")}>
                        <b class={classes!("mr-5")}>{"Emails: "}</b>
                        <p>{format!("{:?}", contact.emails)}</p>
                    </li>

                    <li class={classes!("flex", "flex-row")}>
                        <b class={classes!("mr-5")}>{"Phone Numbers: "}</b>
                        <p>{format!("{:?}", contact.phone_numbers)}</p>
                    </li>


                </ul>
            </div>
        }
        </li>
    }
}
