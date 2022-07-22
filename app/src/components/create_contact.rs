use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_octicons::{Icon, IconKind};

#[function_component(CreateContact)]
pub fn create_contact() -> Html {
    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class={classes!("border-solid", "border-b-2", "items-center", "flex", "flex-col")}>
            <button {onclick} class={classes!("my-5", "border-solid", "border-2", "w-1/2")}>
                <div class={classes!("flex", "flex-row", "justify-center")}>
                    <i class={classes!("mx-3", "mt-1")}>
                        { Icon::new(IconKind::PlusCircle) }</i>
                    {"Create New Contact"}
                </div>
            </button>
            if *toggle {
                <CreateContactForm />
            }
        </div>
    }
}

#[function_component(CreateContactForm)]
fn create_contact_form() -> Html {
    html! {
        <form action="/api/create/form" method="post" class={classes!("flex", "flex-col", "w-1/2")}>
            <label for="firstname">{"First name:"}</label>
            <input type="text" id="firstname" name="firstname" />

            <label for="lastname">{"Last name:"}</label>
            <input type="text" id="lastname" name="lastname" />

            <label for="nickname">{"Nick name:"}</label>
            <input type="text" id="nickname" name="nickname" />

            <label for="company">{"Company:"}</label>
            <input type="text" id="company" name="company" />

            <label for="url">{"Website:"}</label>
            <input type="text" id="url" name="url" />

            <label for="notes">{"Notes:"}</label>
            <input type="text" id="notes" name="notes" />

            <label for="street">{"Street:"}</label>
            <input type="text" id="street" name="street" />

            <label for="city">{"City:"}</label>
            <input type="text" id="city" name="city" />

            <label for="state">{"State:"}</label>
            <input type="text" id="state" name="state" />

            <label for="zip">{"Zip-Code:"}</label>
            <input type="text" id="zip" name="zip" />
            
            <label for="country">{"Country:"}</label>
            <input type="text" id="country" name="county" />

            <label for="emails">{"Emails:"}</label>
            <input type="text" id="emails" name="emails" />

            <label for="phone_numbers">{"Phone Numbers:"}</label>
            <input type="text" id="phone_numbers" name="phone_numbers" />

            <label for="favorite">{"Favorite:"}</label>
            <input type="checkbox" id="favorite" name="favorite" />

            <label for="active">{"Active:"}</label>
            <input type="checkbox" id="active" name="active" />

            <button type="submit" class={classes!("my-5", "border-solid", "border-2")}>
                {"Submit"}
            </button>
        </form>
    }
}
