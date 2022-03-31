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
    cx.render(rsx!(
        div {
            display: "inline-flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            padding:  "0.5rem",
            background_color: "{color}",
            div {
                flex: "1",
                width: "100%",
                height: "1px",
            }
            div {
                margin: "0 1rem",
            }
            div {
                flex: "1",
                width: "100%",
                height: "1px",
            }
        }
    ))
}