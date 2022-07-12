use yew::prelude::*;
//use yew_hooks::prelude::*;

mod input;
mod table;
mod graphql;

use input::InputComp;
use table::TableComp;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <>
            <InputComp />
            <TableComp />
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
