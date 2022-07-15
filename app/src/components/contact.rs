// Components for the contact struct
use yew::prelude::*;
use yew_hooks::prelude::*;
use titlecase::titlecase;

use crate::shared::Contact;

#[derive(Properties, PartialEq)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
}

// creates a list item for each contact in vector
#[function_component(ContactList)]
pub fn contact_list(props: &ContactListProps) -> Html {
    // sort contacts by first name
    let mut contacts: Vec<Contact> = props.contacts.clone(); 
    contacts.sort_by_key(|c| c.firstname.clone());

    html! {
        contacts.iter().map(|contact| {
            html!{
                <li class={classes!("my-1")}>
                    <ContactLink contact={contact.clone()} />
                </li>
            }
        }).collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
struct ContactLinkProps {
    contact: Contact,
}

// make this a struct component so that you can reset the toggle on search
// Makes each list item a button attached to a hidden div with the info
#[function_component(ContactLink)]
fn contact_link(props: &ContactLinkProps) -> Html {
    let contact = &props.contact;

    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <>
        <button {onclick} class={classes!("text-xl", "font-bold")}>
            { format!("{} {}",titlecase(&contact.firstname), titlecase(&contact.lastname)) }
        </button>

        if *toggle {
            <div class={classes!()}>
                <ul class={classes!("text-zinc-400")}>
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
                    <li>{format!("Emails: {:?}", contact.emails)}</li>
                    <li>{format!("Phone Numbers: {:?}", contact.phone_numbers)}</li>
                </ul>
            </div>
        }
        </>
    }
}
