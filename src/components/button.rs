
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ButtonProps {
    text: String,
}

#[warn(non_snake_case)]
pub fn Button(cx: Scope<ButtonProps>) -> Element {
    cx.render(rsx!(
        button {
            "cx.props.text"
        }
    ))
}