use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;
use yew_octicons::{Icon, IconKind};

use crate::components::*;
use crate::shared::*;

#[function_component(Favorites)]
pub fn favorites() -> Html {
    let search_value: UseStateHandle<Option<String>> = use_state(|| None);
    let toggle = use_bool_toggle(false);

    // calls the correct fetch request based on the input value
    let search_value_state = search_value.clone();
    let state = use_async(async move {
        let search_value = search_value_state;
        match &*search_value {
            Some(v) => fetch_contact(v.into()).await,
            None => fetch_all().await,
        }
    });

    let toggle_onclick = {
        let toggle = toggle.clone();
        let search_value = search_value.clone();
        let state = state.clone();
        Callback::from(move |_| {
            toggle.toggle();
            search_value.set(None);
            if !*toggle {
                state.run();
            }
        })
    };

    // saves the input value
    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if input.value() == "" {
                search_value.set(None);
            } else {
                search_value.set(Some(input.value().to_lowercase()));
            }
        })
    };

    // runs query on button press
    let state_onclick = state.clone();
    let refresh_onclick = {
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
        <div class={classes!("border-solid", "border-b-2", "flex", "flex-col", "items-center")}>
            <button onclick={toggle_onclick} class={classes!("my-5", "border-solid", "border-2", "flex-1", "w-1/2")}>
                <div class={classes!("flex", "flex-row", "justify-center")}>
                    <i class={classes!("mx-3", "mt-1")}>
                        { Icon::new(IconKind::HeartFill) }</i>
                    {"Open Favorites"}
                </div>
            </button>

            if *toggle {
            <div class={classes!("flex", "justify-center", "flex-col")}>
                <div class={classes!("flex", "flex-row", "justify-center")}>
                    <i>{ Icon::new_big(IconKind::Search) }</i>
                    <input {oninput} {onkeypress} type="search"
                        class={classes!("ml-2", "border-solid", "border-2", "bg-zinc-700", "text-zinc-200", "mb-5")}/>
                </div>

                <button onclick={refresh_onclick} class={classes!("mb-5", "border-solid", "border-2", "flex-1")}>
                    <div class={classes!("flex", "flex-row", "justify-center")}>
                        <i class={classes!("mx-3", "mt-1")}>
                            { Icon::new(IconKind::Sync) }</i>
                            {"Refresh Favorites"}
                    </div>
                </button>
            </div>

            <div id="contact_list_container"
                class={classes!("flex", "justify-center", "flex-col")}>
            {
                if let Some(contacts) = &state.data {
                    html! {
                        <>
                            <ol class={classes!("flex", "justify-center", "flex-col")}>
                                <CreateContactList contacts={contacts.clone()} list_type={ContactListType::Favorites} state={state.clone()} />
                            </ol>
                        </>
                        }
                } else {
                    html! {}
                }
            }
            </div>
        }
    </div>
    }
}
