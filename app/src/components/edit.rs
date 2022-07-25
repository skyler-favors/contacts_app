use crate::shared::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct EditProps {
    pub contact: Contact,
    pub id: i32,
}

#[function_component(Edit)]
pub fn edit(props: &EditProps) -> Html {
    html! {

    }
}
