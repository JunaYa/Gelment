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
    maxlength: u32,

    #[props(default)]
    minlength: u32,

    #[props(default)]
    value: &'a str,
}

pub fn TextInput<'a>(cx: Scope<'a, TextInputProps<'a>>) -> Element {
    let color = cx.props.color.text_color();
    let font_size = cx.props.size.get_font_size();
    let maxlength = cx.props.maxlength;
    let minlength = cx.props.minlength;
    cx.render(rsx!(
        input {
            "type": "text",
            value: "",
            color: "{color}",
            font_size: "{font_size}",
            maxlength: "{maxlength}",
            minlength: "{minlength}",
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