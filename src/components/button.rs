
use dioxus::{prelude::*, events::MouseEvent};
use crate::{size::Size};

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default)]
    color: String,

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
    cx.render(rsx!(
        button {
            disabled: "{cx.props.disabled}",
            padding: "{padding_val}",
            onclick: move |evt| { cx.props.onclick.call(evt) },
            "{cx.props.text}"
        }
    ))
}