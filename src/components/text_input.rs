use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct TextInputProps {
    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,

    #[props(default)]
    value: &str,
}

pub fn TextInput(cx: Scope<TextInputProps>) -> Element {
    cx.render(rsx!(
        input {
            "type": "text",
            value: "{search_input}",
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