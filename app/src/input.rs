use yew::prelude::*;

#[function_component(InputComp)]
pub fn input_comp() -> Html {
    html! {
        <>
            <h1>{"SQL Project"}</h1>
            <div>
                <label for="first">{"First Name:"}</label>
                <input type="text" id="first" name="first" />

                <label for="last">{"Last Name:"}</label>
                <input type="text" id="last" name="last" />
                
                <button type="button">{"Submit"}</button>
            </div>
        </>
    }
}

