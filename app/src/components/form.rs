use crate::{
    msg_ctx::{Actions, MessageContext},
    shared::Contact,
};
use web_sys::{console::log_1, HtmlInputElement};
use yew::prelude::*;
use itertools::izip;

#[derive(Properties, PartialEq)]
pub struct JsonFormProps {
    pub contact: Contact,
}

#[function_component(JsonForm)]
pub fn json_form(props: &JsonFormProps) -> Html {
    let ctx = use_context::<MessageContext>().unwrap();
    let contact = props.contact.clone();
    let names = vec![
        "firstname", "lastname", "nickname", "company", "url", "notes", 
        "street", "city", "state", "zipcode", "country",
        "emails", "phones",
    ];
    let mut input_states: Vec<UseStateHandle<String>> = Vec::new();
    let data = Contact::to_vec(contact.clone());
    for d in data {
        let state: UseStateHandle<String> = use_state(|| d.to_string());
        input_states.push(state);
    }

    let inputs = izip!(names, input_states.clone())
        .map(|(name, state)| {
            html! {
                <div class={classes!("flex", "flex-col")}>
                    <FormItem {name} {state}/>
                </div>
            }
        })
        .collect::<Html>();


    // Submit button
    let onclick = {
        let ctx = use_context::<MessageContext>().unwrap();
        let input_states = input_states.clone();
        let index = ctx.contacts.iter().position(|x| x.id == contact.id).expect("error");
        Callback::from(move |_| {
            let mut data: Vec<String> = input_states
                .clone()
                .into_iter()
                .map(|state| state.to_string())
                .collect();
            data.push(contact.id.to_string());
            data.push(contact.favorite.to_string());
            data.push(contact.active.to_string());
            let contact = Contact::builder(data);
            ctx.dispatch(Actions::Edit(index, contact))
        })
    };

    html! {
    <div>
        {inputs}
        <button {onclick}>{"Submit"}</button>
    </div>
    }
}

#[derive(Properties, PartialEq)]
struct ListItemProps {
    name: String,
    state: UseStateHandle<String>,
}

#[function_component(FormItem)]
fn form_item(props: &ListItemProps) -> Html {
    let name = &props.name;
    let oninput = {
        let name = name.clone();
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
            log_1(&format!("{}: {}", name, input.value()).into());
        })
    };
    let state = props.state.clone();

    html! {
    <>
    <label for={name.clone()}>{name.clone()}</label>
    <input {oninput} value={state.to_string()} type="text" id={name.clone()} name={name.clone()} />
    </>
    }
}
