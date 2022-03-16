
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ButtonProps<'a> {
    #[props(default)]
    color: String,
    
    text: &'a str,
}

#[warn(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx!(
        button {
            display: "flex",
            padding: "8px 16px",
            "{cx.props.text}"
        }
    ))
}