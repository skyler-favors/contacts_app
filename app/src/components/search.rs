use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew_octicons::{Icon, IconKind};
use crate::shared::delete_all;

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

    let full_delete = use_async(async move {
        delete_all().await
    });

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
            ctx.dispatch(Actions::Filter(Filter::Search(input.value().to_lowercase())))
        })
    };

    html! {
        <>
            <header class={classes!("flex", "flex-row", "justify-center")}>
                <i>{ Icon::new_big(IconKind::Search) }</i>
                <input {oninput} type="search"
                    class={classes!("ml-2", "border-solid", "border-2", 
                        "bg-zinc-700", "text-zinc-200", "mb-5")}/>

                <button onclick={onclick_fav} class={classes!("border-solid", "border-2", "mb-5")}>
                    <div class={classes!("flex", "flex-row", "justify-center")}>
                        <i class={classes!("mx-3", "mt-1")}>
                            { Icon::new(IconKind::HeartFill) }</i>
                        { "Show Favorites" }
                    </div>
                </button>

                <button onclick={onclick_trash} class={classes!("border-solid", "border-2", "mb-5")}>
                    <div class={classes!("flex", "flex-row", "justify-center")}>
                        <i class={classes!("mx-3", "mt-1")}>
                            { Icon::new(IconKind::Trash) }</i>
                        { "Show Trash" }
                    </div>
                </button>

                if *toggle_empty_trash {
                 <button onclick={onclick_empty} class={classes!("border-solid", "border-2", "mb-5")}>
                    <div class={classes!("flex", "flex-row", "justify-center")}>
                        <i class={classes!("mx-3", "mt-1")}>
                            { Icon::new(IconKind::Trash) }</i>
                        { "Empty Trash" }
                    </div>
                </button>
                }

   
            </header>
        </>
    }
}
