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
    border_radius: Size,

    #[props(default)]
    disabled: bool,

    onclick: EventHandler<'a, MouseEvent>,

    #[props(default)]
    text: &'a str,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let padding_val = cx.props.size.get_padding();
    let font_size = cx.props.size.get_font_size();
    let bg_color = cx.props.bg_color.bg_color();
    let color = cx.props.color.text_color();
    let cursor = if cx.props.disabled == true { "not-allowed" } else { "pointer" };
    let border_radius = cx.props.border_radius.get_border_radius();
    cx.render(rsx!(
        button {
            disabled: "{cx.props.disabled}",
            color: "{color}",
            background_color: "{bg_color}",
            font_size: "{font_size}",
            padding: "{padding_val}",
            border: "none",
            cursor: "{cursor}",
            box_shadow: "0.1rem 0.1rem 0.6rem {bg_color}, -0.2rem -0.2rem 0.5rem #FFFFFF",
            border_radius: "{border_radius}",
            onclick: move |evt| { 
                cx.props.onclick.call(evt);
            },
            "{cx.props.text}"
        }
    ))
}
