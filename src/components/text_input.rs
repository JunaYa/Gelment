use dioxus::prelude::*;

#[derive(Props, PartialEq)]
struct TextInputProps {
    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,
}