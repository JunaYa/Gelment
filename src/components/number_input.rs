use dioxus::{prelude::*};

#[derive(Props)]
pub struct NumberInputProps<'a> {

    #[props(default)]
    pub value: i32,

    #[props(default)]
    pub label: &'a str,
}

pub fn NumberInput<'a>(cx: Scope<'a, NumberInputProps<'a>>) -> Element {
    let value = cx.props.value;
    let label = cx.props.label;
    cx.render(rsx!(
        div {
            display: "inline-flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            padding: "0.2rem",
            background_color: "white",
            height: "2rem",
            margin: "0.5rem",
            border_radius: "0.5rem",
            cursor: "pointer",
            div {
                align_items: "flex-start",
                border_radius: "0.5rem",
                background_color: "white",
                margin_right: "0.5rem",
                "{label}"
            }
            input {
                border: "none",
                text_align: "center",
                font_size: "1.5rem",
                font_weight: "bold",
                color: "black",
                background_color: "red",
                text_align: "center",
                oninput: move |evt| {
                    println!("{:?}", evt);
                },
                value: "{value}",
            }
        }
    ))
}