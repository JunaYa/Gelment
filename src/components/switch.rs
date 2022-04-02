use dioxus::{prelude::*, events::MouseEvent};
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct SwitchProps<'a> {
    #[props(default)]
    pub color: Color,

    #[props(default)]
    pub size: Size,

    #[props(default)]
    pub onclick: EventHandler<'a, MouseEvent>,
}

pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element {
    let color = cx.props.color.bg_color();
    let width = cx.props.size.get_switch_width();
    let height = cx.props.size.get_switch_height();
    cx.render(rsx!(
        div {
            display: "inline-flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            padding: "0.5rem",
            background_color: "{color}",
            width: "{width}",
            height: "{height}",
            margin: "0.5rem",
            border_radius: "{height} {height} {height} {height}",
            div {
                align_items: "flex-start",
                width: "{height}",
                height: "{height}",
                border_radius: "50%",
                background_color: "white",
            }
        }
    ))
}