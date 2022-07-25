use yew::prelude::*;
use yew_hooks::use_bool_toggle;
use crate::form::JsonForm;

#[function_component(Create)]
pub fn create() -> Html {
    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| {
            toggle.toggle();
        })
    };

    html! {
        <div>
            <button {onclick} class={classes!("border-solid", "border-2", "mb-5")}>{"Create Contact"}</button>
        if *toggle {
            <JsonForm />
        }
        </div>
    }
}

