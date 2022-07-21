use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::*;
use crate::shared::*;

async fn fetch_trash() -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>("http://localhost:8000/api/read/trash".into()).await
}

#[function_component(Trashcan)]
pub fn trash() -> Html {
    let state = use_async(async move { fetch_trash().await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <>
        <button {onclick} class={classes!("my-5", "border-solid", "border-2")}>
            {"Open Trash"}
        </button>

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
                            <CreateContactList contacts={contacts.clone()} trash={true} state={state.clone()} />
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
