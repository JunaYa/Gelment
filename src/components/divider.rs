use dioxus::{prelude::*, events::MouseEvent};
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct DividerProps<'a> {
    #[props(default)]
    pub color: Color,

    #[props(default)]
    pub size: Size,

    #[props(default)]
    pub border_style: &'a str,

    #[props(default)]
    pub onclick: EventHandler<'a, MouseEvent>,
}

pub fn Divider<'a>(cx: Scope<'a, DividerProps<'a>>) -> Element {
    let color = cx.props.color.bg_color();
    let mut border_style = cx.props.border_style;
    if border_style.is_empty() {
        border_style = "solid";
    }
    cx.render(rsx!(
        div {
            width: "100%",
            height: "1px",
            border_top: "1px {border_style} {color}",
            margin: "1rem 0",
        }
    ))
}