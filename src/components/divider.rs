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

    #[props(default)]
    pub text: &'a str,
}

pub fn Divider<'a>(cx: Scope<'a, DividerProps<'a>>) -> Element {
    let color = cx.props.color.bg_color();
    let mut border_style = cx.props.border_style;
    let text = cx.props.text;
    if border_style.is_empty() {
        border_style = "solid";
    }
    cx.render(rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            padding:  "0.5rem",
            div {
                flex: "1",
                width: "100%",
                height: "1px",
                border_top: "1px {border_style} {color}",
            }
            div {
                margin: "0 1rem",
                "{text}",
            }
            div {
                flex: "1",
                width: "100%",
                height: "1px",
                border_top: "1px {border_style} {color}",
            }
        }
    ))
}