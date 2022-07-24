use crate::components::*;
use crate::shared::Contact;
use std::rc::Rc;
use yew::prelude::*;

pub enum Actions {
    Push(Filter),
    Update(Rc<Vec<Rc<Contact>>>),
}

#[derive(Clone, PartialEq)]
pub struct AppContext {
    pub contacts: Rc<Vec<Rc<Contact>>>,
    pub filters: Rc<Vec<Filter>>,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            contacts: Rc::new(Vec::new()),
            filters: Rc::new(Vec::new()),
        }
    }
}

impl Reducible for AppContext {
    type Action = Actions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctx = match action {
            Actions::Push(f) => {
                let mut new_filters: Vec<Filter> = self.filters.to_vec();
                let index = self.filters.iter().position(|x| x == &f);
                match index {
                    Some(i) => {
                        new_filters.remove(i);
                    }
                    None => {
                        new_filters.push(f);
                    }
                }

                AppContext {
                    contacts: self.contacts.clone(),
                    filters: Rc::new(new_filters),
                }
            }
            Actions::Update(v) => AppContext {
                contacts: v,
                filters: self.filters.clone(),
            },
        };
        Rc::new(next_ctx)
    }
}

pub type MessageContext = UseReducerHandle<AppContext>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    pub contacts: Rc<Vec<Rc<Contact>>>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MessageProvider)]
pub fn message_provider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| AppContext {
        contacts: props.contacts.clone(),
        filters: Rc::new(Vec::new()),
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}
