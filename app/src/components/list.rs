use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;

use crate::shared::*;
use crate::components::*;

async fn fetch_contacts() -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>("http://localhost:8000/api/read/all".into()).await
}

async fn fetch_contact(name: String) -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>(format!("http://localhost:8000/api/read/name/{}", name)).await
}

#[function_component(ListContacts)]
pub fn list_contacts() -> Html {
    let search_value: UseStateHandle<Option<String>> = use_state(|| None);

    let search_value_state = search_value.clone();
    let state = use_async(async move {
        let search_value = search_value_state;
        match &*search_value {
            Some(v) => {
                fetch_contact(v.into()).await
            },
            None => {
                fetch_contacts().await
            },
        }
    });

    //let search_value_input = search_value.clone();
    let search_value_input = search_value;
    let oninput = {
        let search_value = search_value_input;
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if input.value() == "" {
                search_value.set(None);
            } else {
                search_value.set(Some(input.value()));
            }
        })
    };

    let state_onclick = state.clone();
    let onclick = {
        Callback::from(move |_| {
            // You can manually trigger to run in callback or use_effect.
            state_onclick.run();
        })
    };

    let state_keypress = state.clone();
    let onkeypress: Callback<web_sys::KeyboardEvent> = {
        Callback::from(move |key: web_sys::KeyboardEvent| {
            if key.key() == "Enter" {
                state_keypress.run();
            }
        })
    };

    html! {
        <>
            <input {oninput} {onkeypress} type="search" />
            <button {onclick} disabled={state.loading}>{ "Load Contacts" }</button>

            <p>
                {
                    if state.loading {
                        html! { "Loading, wait a sec..." }
                    } else {
                        html! {}
                    }
                }
            </p>

            {
                if let Some(contacts) = &state.data {
                    html! {
                        <>
                            <ol>
                                <ContactList contacts={contacts.clone()} />
                            </ol>
                        </>
                        }
                } else {
                    html! {}
                }
            }

            <p>
                {
                    if let Some(error) = &state.error {
                        html!{<p>{format!("{:?}", error)}</p>}
                    } else {
                        html! {}
                    }
                }
            </p>
        </>
    }
}
