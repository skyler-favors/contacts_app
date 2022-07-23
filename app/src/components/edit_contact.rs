use crate::shared::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct EditProps {
    pub contact: Contact,
    pub id: i32,
}

#[function_component(Edit)]
pub fn edit(props: &EditProps) -> Html {
    let contact = &props.contact.clone();
    let action = format!("/api/update/form/id/{}", props.id);
    html! {
    <form {action} method="post" class={classes!("flex", "flex-col", "w-1/2")}>
        <label for="firstname">{"First name:"}</label>
        <input value={contact.firstname.clone()} type="text" id="firstname" name="firstname" />

        <label for="lastname">{"Last name:"}</label>
        <input value={contact.lastname.clone()} type="text" id="lastname" name="lastname" />

        <label for="nickname">{"Nick name:"}</label>
        <input value={contact.nickname.clone()} type="text" id="nickname" name="nickname" />

        <label for="company">{"Company:"}</label>
        <input value={contact.company.clone()} type="text" id="company" name="company" />

        <label for="url">{"Website:"}</label>
        <input value={contact.url.clone()} type="text" id="url" name="url" />

        <label for="notes">{"Notes:"}</label>
        <input value={contact.notes.clone()} type="text" id="notes" name="notes" />

        <label for="street">{"Street:"}</label>
        <input value={contact.street.clone()} type="text" id="street" name="street" />

        <label for="city">{"City:"}</label>
        <input value={contact.city.clone()} type="text" id="city" name="city" />

        <label for="state">{"State:"}</label>
        <input value={contact.state.clone()} type="text" id="state" name="state" />

        <label for="zip">{"Zip-Code:"}</label>
        <input value={contact.zip.clone()} type="text" id="zip" name="zip" />

        <label for="country">{"Country:"}</label>
        <input value={contact.country.clone()} type="text" id="country" name="county" />

        <label for="emails">{"Emails:"}</label>
        <input value={contact.emails.join(" ")} type="text" id="emails" name="emails" />

        <label for="phone_numbers">{"Phone Numbers:"}</label>
        <input value={contact.phone_numbers.join(" ")} type="text" id="phone_numbers" name="phone_numbers" />

        <label for="favorite">{"Favorite:"}</label>
        <input checked={contact.favorite} type="checkbox" id="favorite" name="favorite" />

        <label for="active">{"Active:"}</label>
        <input checked={contact.active} type="checkbox" id="active" name="active" />

        <button type="submit" class={classes!("my-5", "border-solid", "border-2")}>
            {"Submit"}
        </button>
    </form>    }
}
