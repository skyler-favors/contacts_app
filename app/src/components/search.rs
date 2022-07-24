use yew::prelude::*;
use yew_octicons::{Icon, IconKind};
use super::Actions;
use yew_hooks::prelude::*;

use super::MessageContext;

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
            ctx.dispatch(Actions::Push(Filter::Favorites))
        })
    };

    let toggle_empty_trash = use_bool_toggle(false);
let onclick_trash = {
        let ctx = msg_ctx;
        let toggle = toggle_empty_trash.clone();
        Callback::from(move |_| {
            ctx.dispatch(Actions::Push(Filter::Trash));
            toggle.toggle();
        })
    };

    html! {
        <>
            <header class={classes!("flex", "flex-row", "justify-center")}>
                <i>{ Icon::new_big(IconKind::Search) }</i>
                <input type="search"
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
                 <button class={classes!("border-solid", "border-2", "mb-5")}>
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
