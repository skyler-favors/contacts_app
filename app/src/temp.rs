
#[derive(PartialEq)]
enum SearchValue {
    Id(i32),
    Name(String),
}

#[derive(Properties, PartialEq)]
struct ListContactsProps {
    search_value: Option<SearchValue>,
}

#[function_component(ListContacts)]
fn list_contacts(props: &ListContactsProps) -> Html {
    match &props.search_value {
        Some(search_value) => {
            // If value is given run search
            match search_value {
                SearchValue::Id(id) => {
                    html!{
                        <h1>{format!("id: {}", id)}</h1>
                    }
                },
                SearchValue::Name(name) => {
                    html!{
                        <h1>{format!("name: {}", name)}</h1>
                    }
                },
            }
        },
        None => {
            // List every contact if no value

            html! {
            }
        }
    }
}
