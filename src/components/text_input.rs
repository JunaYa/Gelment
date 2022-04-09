use dioxus::prelude::*;
use crate::size::Size;
use crate::color::Color;

#[derive(Props, PartialEq)]
pub struct TextInputProps<'a> {
    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,

    #[props(default)]
    value: &'a str,

    #[props(default)]
    label: &'a str,
}

pub fn TextInput<'a>(cx: Scope<'a, TextInputProps<'a>>) -> Element {
    cx.render(rsx!(
        input {
            "type": "text",
            value: "",
            placeholder: "placeholder",
            oninput: move |evt| {
                println!("{:?}", evt);
            },
            onkeydown: move |evt| {
                println!("{:?}", evt);
                if evt.key == "Enter" {
                }
            }
        }
    ))
}