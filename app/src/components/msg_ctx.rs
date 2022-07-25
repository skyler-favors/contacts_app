use crate::components::*;
use crate::shared::Contact;
use std::rc::Rc;
use yew::prelude::*;

pub enum Actions {
    Filter(Filter),
    Favorite(Contact),
    Trash(Contact),
    Delete,
    Create(Contact),
    Edit(usize, Contact),
}

#[derive(Clone, PartialEq, Default)]
pub struct AppContext {
    pub contacts: Vec<Contact>,
    pub filters: Vec<Filter>,
}

impl Reducible for AppContext {
    type Action = Actions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let contacts = Rc::new(self.contacts.clone());
        let filters = Rc::new(self.filters.clone());
        let mut new_filters: Vec<Filter> = self.filters.clone();
        let mut new_contacts: Vec<Contact> = self.contacts.clone();


        match action {
            Actions::Filter(f) => {
                let index = filters.iter().position(|x| match x {
                    Filter::Favorites => x == &f,
                    Filter::Trash => x == &f,
                    Filter::Search(_old) => {
                        matches!(&f, Filter::Search(_new))
                    }
                });
                match &f {
                    Filter::Search(new) => match index {
                        Some(i) => {
                            if new.is_empty() {
                                new_filters.remove(i);
                            } else {
                                new_filters.remove(i);
                                new_filters.push(f);
                            }
                        }
                        None => {
                            new_filters.push(f);
                        }
                    },
                    _ => match index {
                        Some(i) => {
                            new_filters.remove(i);
                        }
                        None => {
                            new_filters.push(f);
                        }
                    },
                }
            }
            Actions::Trash(c) => {
                let index = contacts.iter().position(|x| x == &c).expect("error");
                new_contacts[index].active = !contacts[index].active;
            },
            Actions::Favorite(c) => {
                let index = contacts.iter().position(|x| x == &c).expect("error");
                new_contacts[index].favorite = !contacts[index].favorite;
            },
            Actions::Edit(i, c) => {
                new_contacts[i] = c;
            },
            Actions::Create(c) => {
                new_contacts.push(c);
            },
            Actions::Delete => {
                new_contacts = new_contacts.clone().into_iter().filter(|x| x.active).collect();
            }
        }

        let new_ctx = AppContext {
            contacts: new_contacts,
            filters: new_filters,
        };

        Rc::new(new_ctx)
    }
}

pub type MessageContext = UseReducerHandle<AppContext>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    pub contacts: Vec<Contact>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MessageProvider)]
pub fn message_provider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| AppContext {
        contacts: props.contacts.clone(),
        filters: Vec::new(),
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}
