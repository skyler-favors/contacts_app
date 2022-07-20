use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;

use crate::shared::Contact;

#[function_component(CreateContact)]
pub fn create_contact() -> Html {
    let toggle = use_bool_toggle(false);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <>
            <button {onclick} class={classes!("my-5", "border-solid", "border-2")}>
                {"Create New Contact"}
            </button>

            if *toggle {
                <div class={classes!("flex", "flex-col")}>
                    <h1>{"Create Contact Here"}</h1>
                    <CreateContactForm />
                </div>
            }
        </>
    }
}

#[function_component(CreateContactForm)]
fn create_contact_form() -> Html {
    // used to disable the submit if non-nullable values are null
//    let toggle = use_bool_toggle(true);
//    let toggle_event = {
//        let toggle = toggle.clone();
//        Callback::from(move |_| toggle.toggle())
//    };

    // stores values for contact
    let state_value_strings: Vec<UseStateHandle<Option<String>>> = vec![use_state(|| None); 11];
    let value_string_names = vec!["firstname", "lastname", "nickname", "company", "website", "notes", 
        "street", "city", "state", "zip", "country"];

    // stores emails and phones
    let state_value_vecs: Vec<UseStateHandle<Option<Vec<String>>>> = vec![use_state(|| None); 2];
    let value_vec_names = vec!["emails", "phone_numbers"];

    // store bool values
    let state_value_bools: Vec<UseStateHandle<Option<bool>>> = vec![use_state(|| None); 2];
    let value_bool_names = vec!["active", "favorite"];

    // create the html list of inputs for strings
    let svs = state_value_strings.clone();
    let input_strings_list = svs.iter().zip(value_string_names.iter()).map(|(state, name)| {
        let state = state.clone();
        html! {
            <InputDataStrings name={name.to_string()} {state} />
        }
    }).collect::<Html>();

    // bools
    let svv = state_value_vecs.clone();
    let input_vecs_list = svv.iter().zip(value_vec_names.iter()).map(|(state, name)| {
        let state = state.clone();
        html! {
            <InputDataVecs name={name.to_string()} {state} />
        }
    }).collect::<Html>();

    // Vecs
    let svb = state_value_bools.clone();
    let input_bools_list = svb.iter().zip(value_bool_names.iter()).map(|(state, name)| {
        let state = state.clone();
        html! {
            <InputDataBools name={name.to_string()} {state} />
        }
    }).collect::<Html>();

    // collect data from each input, create json, run POST request
    let onclick = {
        Callback::from(move |_| {
            let mut value_is_none = false;
            state_value_strings.iter().map(|val| {
                if val.is_none() {
                    value_is_none = true;
                }
            });

            state_value_vecs.iter().map(|val| {
                if val.is_none() {
                    value_is_none = true;
                }
            });

            state_value_bools.iter().map(|val| {
                if val.is_none() {
                    value_is_none = true;
                }
            });

            if !value_is_none {
                let contact = Contact {
                    // person table
                    firstname: state_value_strings[0].unwrap(),
                    lastname: state_value_strings[1].unwrap(),
                    nickname: state_value_strings[2].unwrap(),
                    company: state_value_strings[3].unwrap(),
                    url: state_value_strings[4].unwrap(),
                    notes: state_value_strings[5].unwrap(),
                    street: state_value_strings[6].unwrap(),
                    city: state_value_strings[7].unwrap(),
                    state: state_value_strings[8].unwrap(),
                    zip: state_value_strings[9].unwrap(),
                    country: state_value_strings[10].unwrap(),
                    //bools
                    favorite: state_value_bools[0].unwrap(),
                    active: state_value_bools[1].unwrap(),
                    //vecs
                    emails: state_value_vecs[0].unwrap(),
                    phone_numbers:state_value_vecs[1].unwrap(),
                };
            }
        })
    };

    html! {
        <>
            {input_strings_list}
            {input_vecs_list}
            {input_bools_list}
            <button {onclick} class={classes!("my-5", "border-solid", "border-2")}>
                {"Submit"}
            </button>

        </>
    }
}

#[derive(Properties, PartialEq)]
struct InputDataStringsProps {
    name: String,
    state: UseStateHandle<Option<String>>,
}

#[function_component(InputDataStrings)]
fn input_data_strings(props: &InputDataStringsProps) -> Html {
    let name = props.name.clone();
    let state = props.state.clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if input.value() == "" {
                state.set(None);
            } else {
                state.set(Some(input.value().to_lowercase()));
            }
        })
    };

    html! {
        <>
            <div>
                <label>{name}</label>
                <input oninput={oninput} class={classes!("border-solid", "border-2", 
                    "mx-20", "my-1", "bg-zinc-700", "text-zinc-200")}/>
            </div>
        </>
    }
}


#[derive(Properties, PartialEq)]
struct InputDataVecsProps {
    name: String,
    state: UseStateHandle<Option<Vec<String>>>,
}

#[function_component(InputDataVecs)]
fn input_data_vecs(props: &InputDataVecsProps) -> Html {
    let name = props.name.clone();
    let state = props.state.clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            state.set(Some(vec![input.value()]));
        })
    };

    html! {
        <>
            <div>
                <label>{name}</label>
                <input {oninput} class={classes!("border-solid", "border-2", 
                    "mx-20", "my-1", "bg-zinc-700", "text-zinc-200")}/>
            </div>
        </>
    }
}


#[derive(Properties, PartialEq)]
struct InputDataBoolsProps {
    name: String,
    state: UseStateHandle<Option<bool>>,
}

#[function_component(InputDataBools)]
fn input_data_bools(props: &InputDataBoolsProps) -> Html {
    let name = props.name.clone();
    let state = props.state.clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let data = input.value().parse::<bool>();
            match data {
                Ok(b) => {
                    state.set(Some(b));
                },
                Err(e) => {
                },
            }
        })
    };

    html! {
        <>
            <div>
                <label>{name}</label>
                <input {oninput} class={classes!("border-solid", "border-2", 
                    "mx-20", "my-1", "bg-zinc-700", "text-zinc-200")}/>
            </div>
        </>
    }
}
