use yew::prelude::*;
use yew_octicons::{IconKind, Icon};

#[derive(Properties, PartialEq, Clone)]
pub struct HoverIconProps {
    pub icon: IconKind,
    pub text: Option<String>,
}

#[function_component(HoverIcon)]
pub fn hover_icon(props: &HoverIconProps) -> Html {
    html! {
        <div class={classes!("sidebar-icon", "group")}>
            {Icon::new_big(props.icon)}

            if let Some(text) = &props.text {
                <span class={classes!("sidebar-tooltip", "group-hover:visible")}>
                    {text}
                </span>
            }
        </div>
    }
}
