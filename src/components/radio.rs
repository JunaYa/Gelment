use dioxus::{prelude::*, events::MouseEvent};
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct RadioProps<'a> {
    #[props(default)]
    checked: bool,

    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,
    
    #[props(default)]
    disabled: bool,

    onclick: EventHandler<'a, MouseEvent>,
}


pub fn Radio<'a>(cx: Scope<'a, RadioProps<'a>>) -> Element {
    let c_default = Color::Gray;
    let color = if cx.props.checked { cx.props.color.text_color() } else { c_default.text_color()};
    let cursor = if cx.props.disabled { "not-allowed" } else { "pointer" };
    cx.render(rsx!(
        div {
            display: "inline-flex",
            width: ".8rem",
            height: ".8rem",
            background_color: "#fff",
            border: ".15rem solid {color}",
            border_radius: "50%",
            cursor: "{cursor}",
            onclick: move |evt| {
                cx.props.onclick.call(evt);
            }
        }
    ))
}

