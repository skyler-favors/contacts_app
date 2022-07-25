use crate::shared::Contact;
use crate::components::*;
use std::rc::Rc;
use crate::shared::fetch_all;
use yew::prelude::*;
use msg_ctx::MessageProvider;

pub enum FetchState {
    NotFetching,
    Fetching,
    Success(Vec<Contact>),
    Failed(Rc<reqwest::Error>),
}

pub enum Msg {
    ContactFetchState(FetchState),
    GetContacts,
}

pub struct Root {
    contacts: FetchState,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            contacts: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContactFetchState(fetch_state) => {
                self.contacts = fetch_state;
                true
            }
            Msg::GetContacts => {
                ctx.link().send_future(async {
                    match fetch_all().await {
                        Ok(x) => Msg::ContactFetchState(FetchState::Success(x)),
                        Err(e) => Msg::ContactFetchState(FetchState::Failed(e)),
                    }
                });
                ctx.link().send_message(Msg::ContactFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.contacts {
            FetchState::NotFetching => {
                // auto fetch on load
                ctx.link().send_message(Msg::GetContacts);
                html! {"Not Fetching"}
            },

            FetchState::Fetching => html! {"Loading"},

            FetchState::Success(contacts) => {
        
                html! {
                    <MessageProvider contacts={contacts.clone()}>
                        <Create />
                        <Search />
                        <List />
                    </MessageProvider>
                }
            },

            FetchState::Failed(error) => html! {
                <>
                    <h1>{"Failed to load"}</h1>
                    <h2>{error}</h2>
                    <button onclick={ctx.link().callback(|_| Msg::GetContacts)} >
                        {"Try again?"}
                    </button>
                </>
            },
        }
    }
}
