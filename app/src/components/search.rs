use crate::shared::delete_all;
use crate::JsonForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_octicons::{Icon, IconKind};
use crate::HoverIcon;

use super::msg_ctx::{Actions, MessageContext};
use yew_hooks::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    Search(String),
    Favorites,
    Trash,
}

#[function_component(Search)]
pub fn search() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let onclick_fav = {
        let ctx = msg_ctx.clone();
        Callback::from(move |_| {
            ctx.dispatch(Actions::Filter(Filter::Favorites))
        })
    };

    let toggle_empty_trash = use_bool_toggle(false);
    let onclick_trash = {
        let ctx = msg_ctx.clone();
        let toggle = toggle_empty_trash.clone();
        Callback::from(move |_| {
            ctx.dispatch(Actions::Filter(Filter::Trash));
            toggle.toggle();
        })
    };

    let full_delete = use_async(async move { delete_all().await });

    let onclick_empty = {
        let ctx = msg_ctx.clone();
        Callback::from(move |_| {
            full_delete.run();
            ctx.dispatch(Actions::Delete);
        })
    };

    // saves the input value
    let oninput = {
        let ctx = msg_ctx;
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ctx.dispatch(Actions::Filter(Filter::Search(
                input.value().to_lowercase(),
            )))
        })
    };

    let toggle_create = use_bool_toggle(false);

    let onclick_create = {
        let toggle = toggle_create.clone();
        Callback::from(move |_| {
            toggle.toggle();
        })
    };

    html! {
        <>
            <header class={classes!("flex", "flex-row", "justify-between")}>
                <div class={classes!("justify-self-center", "flex", "flex-col", "ml-5")}>
                    <div class={classes!("flex", "flex-row", "justify-self-center")}>
                        <i class={classes!()}>{ Icon::new_big(IconKind::Search) }</i>
                        <input {oninput} type="search"
                            class={classes!("ml-2", "border-solid", "border-2")}/>
                    </div>

                    <div class={classes!("grid", "grid-cols-3", "gap-4")}>
        
                        <button onclick={onclick_fav} class={classes!("col-start-1")}>
                            <HoverIcon icon={IconKind::HeartFill} text={"Filter Favorites"} />
                        </button>

                        <button onclick={onclick_trash} class={classes!("col-start-2")}>
                            <HoverIcon icon={IconKind::Trash} text={"Filter Trash"} />
                        </button>

                        if *toggle_empty_trash {
                         <button onclick={onclick_empty} class={classes!("col-start-3")}>
                            <HoverIcon icon={IconKind::Flame} text={"Empty Trash"} />
                        </button>
                        }
                    </div>
                </div>

                <button onclick={onclick_create} class={classes!("mr-5")}>
                    <HoverIcon icon={IconKind::PersonAdd} text={"Create New Contact"} />
                </button>
            </header>
            if *toggle_create {
                <JsonForm />
            }
        </>
    }
}

