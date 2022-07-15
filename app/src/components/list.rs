use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::*;
use crate::shared::*;

async fn fetch_contacts() -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>("http://localhost:8000/api/read/all".into()).await
}

async fn fetch_contact(name: String) -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>(format!("http://localhost:8000/api/read/name/{}", name)).await
}

// the list container that holds contact components
#[function_component(ListContacts)]
pub fn list_contacts() -> Html {
    // holds the value thats typed into the input
    let search_value: UseStateHandle<Option<String>> = use_state(|| None);

    // calls the correct fetch request based on the input value
    let search_value_state = search_value.clone();
    let state = use_async(async move {
        let search_value = search_value_state;
        match &*search_value {
            Some(v) => fetch_contact(v.into()).await,
            None => fetch_contacts().await,
        }
    });

    // saves the input value
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

    // runs query on button press
    let state_onclick = state.clone();
    let onclick = {
        Callback::from(move |_| {
            // You can manually trigger to run in callback or use_effect.
            state_onclick.run();
        })
    };

    // same as above but on enter press
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
            <div id="input_container" class={classes!("flex", "justify-center", "flex-col")}>
                <input {oninput} {onkeypress} type="search"
                    class={classes!("border-solid", "border-2", "m-10", "bg-zinc-700", "text-zinc-200")}/>
                <button {onclick} disabled={state.loading} class={classes!("border-solid", "border-2")}>
                    { "Load Contacts" }
                </button>
            </div>

            <div id="state_loading_container"
                class={classes!("flex", "justify-center", "flex-col")}>
                <p class={classes!("flex", "justify-center", "flex-col")}>
                    {
                        if state.loading {
                            html! { "Loading, wait a sec..." }
                        } else {
                            html! {}
                        }
                    }
                </p>
            </div>

            <div id="contact_list_container"
                class={classes!("flex", "justify-center", "flex-col")}>
            {
                if let Some(contacts) = &state.data {
                    html! {
                        <>
                            <ol class={classes!("flex", "justify-center", "flex-col")}>
                                <ContactList contacts={contacts.clone()} />
                            </ol>
                        </>
                        }
                } else {
                    html! {}
                }
            }
            </div>

            <div id="error_state_container"
                class={classes!("flex", "justify-center", "flex-col")}>
                <p>
                    {
                        if let Some(error) = &state.error {
                            html!{<p>{format!("{:?}", error)}</p>}
                        } else {
                            html! {}
                        }
                    }
                </p>
            </div>
        </>
    }
}
