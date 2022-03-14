use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct WeButtonProps {
    text: String,
}

pub fn app(cx: Scope<WeButtonProps>) -> Element {
    // use_state
    cx.render(rsx!(
        button {
            color: "balck",
            "{cx.props.text}"
        }
    ))
}