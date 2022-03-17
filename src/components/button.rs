use crate::size::Size;
use crate::color::Color;
use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default)]
    color: Color,

    #[props(default)]
    bg_color: Color,

    #[props(default)]
    size: Size,

    #[props(default)]
    disabled: bool,

    onclick: EventHandler<'a, MouseEvent>,

    #[props(default)]
    text: &'a str,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let padding_val = cx.props.size.get_padding();
    let font_size = cx.props.size.get_font_size();
    let bg_color = cx.props.bg_color.text_color();
    let color = cx.props.color.text_color();
    cx.render(rsx!(
        button {
            disabled: "{cx.props.disabled}",
            background_color: "{bg_color}",
            color: "{color}",
            font_size: "{font_size}",
            padding: "{padding_val}",
            border: "none",
            onclick: move |evt| { cx.props.onclick.call(evt) },
            "{cx.props.text}"
        }
    ))
}
