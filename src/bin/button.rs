use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ButtonProps {
    text: String,
}

fn app(cx: Scope<ButtonProps>) -> Element {
    cx.reader(rsx!(
        button {
            "cx.props.text"
        }
    ))
}