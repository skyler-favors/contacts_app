use crate::{
    msg_ctx::{Actions, MessageContext},
    shared::{create_contact, update_contact, Contact, ContactNullables},
};
use itertools::izip;
use std::rc::Rc;
use web_sys::{console::log_1, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};

#[derive(Properties, PartialEq)]
pub struct JsonFormProps {
    pub contact: Option<Contact>,
}

#[function_component(JsonForm)]
pub fn json_form(props: &JsonFormProps) -> Html {
    let ctx = use_context::<MessageContext>().unwrap();
    let contact = props.contact.clone();
    let names = vec![
        "firstname",
        "lastname",
        "nickname",
        "company",
        "url",
        "notes",
        "street",
        "city",
        "state",
        "zipcode",
        "country",
        "emails",
        "phones",
    ];
    let mut input_states: Vec<UseStateHandle<String>> = Vec::new();

    match &contact {
        Some(c) => {
            let data = Contact::to_vec(c.clone());
            for d in data {
                let state: UseStateHandle<String> = use_state(|| d.to_string());
                input_states.push(state);
            }
        }
        None => {
            let names = names.clone();
            for _n in names {
                let state: UseStateHandle<String> = use_state(|| "".to_string());
                input_states.push(state);
            }
        }
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

    // used for post request
    let contact_state: UseStateHandle<Option<Contact>> = use_state(|| None);

    match contact {
        Some(c) => {
            // send update request
            let contact_state_inner = contact_state.clone();
            let send_update: UseAsyncHandle<(), Rc<reqwest::Error>> = use_async(async move {
                match &*contact_state_inner {
                    Some(x) => {
                        let new = ContactNullables::new(x);
                        update_contact(&new).await?;
                    }
                    None => log_1(&"Missing contact state data".into()),
                }
                Ok(())
            });

            // Submit button
            let onclick = {
                let ctx = use_context::<MessageContext>().unwrap();
                let input_states = input_states.clone();
                let index = ctx
                    .contacts
                    .iter()
                    .position(|x| x.id == c.id)
                    .expect("error");
                Callback::from(move |_| {
                    let mut data: Vec<String> = input_states
                        .clone()
                        .into_iter()
                        .map(|state| state.to_string())
                        .collect();
                    data.push(c.id.to_string());
                    data.push(c.favorite.to_string());
                    data.push(c.active.to_string());
                    let contact = Contact::builder(data);
                    contact_state.set(Some(contact.clone()));
                    ctx.dispatch(Actions::Edit(index, contact));
                    send_update.run();
                })
            };
            html! {
            <div>
                {inputs}
                <div class={classes!("flex", "justify-center")}>
                    <button class={classes!("border-solid", "border-2", "mb-5", "flex-1/3", "font-bold", "mt-3")} {onclick}>{"Submit"}</button>
                </div>
            </div>
            }
        }
        None => {
            let send_create: UseAsyncHandle<(), Rc<reqwest::Error>> = use_async(async move {
                let mut data: Vec<String> = input_states
                    .clone()
                    .into_iter()
                    .map(|state| state.to_string())
                    .collect();

                data.push("0".to_string());
                data.push("false".to_string());
                data.push("true".to_string());
                let mut contact = Contact::builder(data);
                let new = ContactNullables::new(&contact);

                let id = create_contact(&new).await?;
                contact.id = id;
                log_1(&format!("{}",contact.id).into());
                contact_state.set(Some(contact.clone()));
                ctx.dispatch(Actions::Create(contact));

                Ok(())
            });

            // Submit button
            let onclick = {
                let _ctx = use_context::<MessageContext>().unwrap();
                Callback::from(move |_| {
                    send_create.run();
                })
            };

            html! {
            <div>
                {inputs}
                <div class={classes!("flex", "flex-row", "justify-center")}>
                    <button class={classes!("border-solid", "border-2", "mb-5", "mt-3")} {onclick}>{"Submit"}</button>
                </div>
            </div>
            }
        }
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
