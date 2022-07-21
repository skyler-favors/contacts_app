// Components for the contact struct
use yew::prelude::*;
use yew_hooks::prelude::*;
use titlecase::titlecase;

use crate::shared::*;

#[derive(Properties, PartialEq)]
pub struct CreateContactListProps {
    pub contacts: Vec<Contact>,
    pub trash: bool,
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
                <li class={classes!("my-1")}>
                    <ContactLink trash={props.trash} contact={contact.clone()} state={props.state.clone()} />
                </li>
            }
        }).collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
struct ContactLinkProps {
    contact: Contact,
    trash: bool,
    state: UseAsyncHandle<Vec<Contact>, Error>,
}

// make this a struct component so that you can reset the toggle on search
// Makes each list item a button attached to a hidden div with the info
#[function_component(ContactLink)]
fn contact_link(props: &ContactLinkProps) -> Html {
    let contact = &props.contact;
    let id = contact.id;

    if !contact.active && !props.trash {
        return html! {}
    }

    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let mut lastname = "".to_string();
    match &contact.lastname {
        Some(last) => lastname = titlecase(last),
        None => {},
    }

    let reload = props.state.clone();
    let mut state = use_async(async move { 
        let result = toggle_delete_false(id).await;
        reload.run();
        match result {
            Ok(_x) => Ok(()),
            Err(e) => Err(e),
        }
    });

    let reload = props.state.clone();
    if props.trash {
        state = use_async(async move { 
        let result = toggle_delete_true(id).await; 
        reload.run();
        match result {
            Ok(_x) => Ok(()),
            Err(e) => Err(e),
        }
        });
    }

    let trash_onclick = {
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <>
        <div>
            <button {onclick} class={classes!("text-xl", "font-bold")}>
                { format!("{} {}",titlecase(&contact.firstname), lastname) }
            </button>

            <button onclick={trash_onclick} class={classes!("mx-3", "float-right")}>
                {"Remove"}
            </button>

            <button class={classes!("mx-3", "float-right")}>
                {"Favorite"}
            </button>

            <button class={classes!("mx-3", "float-right")}>
                {"Edit"}
            </button>
        </div>

        if *toggle {
            <div class={classes!()}>
                <ul class={classes!("text-zinc-400")}>
                    if let Some(nick) = &contact.nickname {
                        <li>{format!("Nickname: {}", nick)}</li>
                    }

                    if let Some(company) = &contact.company {
                        <li>{format!("Company: {}", company)}</li>
                    }

                    if let Some(url) = &contact.url {
                        <li>{format!("Website: {}", url)}</li>
                    }

                    if let Some(notes) = &contact.notes {
                        <li>{format!("Notes: {}", notes)}</li>
                    }

                    <li>{format!("Favorite: {}", &contact.favorite)}</li>
                    <li>{format!("Active: {}", &contact.active)}</li>

                    if let Some(street) = &contact.street {
                        <li>{format!("Street: {}", street)}</li>
                    }

                    if let Some(city) = &contact.city {
                        <li>{format!("City: {}", city)}</li>
                    }

                    if let Some(state) = &contact.state {
                        <li>{format!("State: {}", state)}</li>
                    }

                    if let Some(zip) = &contact.zip {
                        <li>{format!("Zip-Code: {}", zip)}</li>
                    }

                    if let Some(country) = &contact.country {
                        <li>{format!("Country: {}", country)}</li>
                    }
                    <li>{format!("Emails: {:?}", contact.emails)}</li>
                    <li>{format!("Phone Numbers: {:?}", contact.phone_numbers)}</li>
                </ul>
            </div>
        }
        </>
    }
}
