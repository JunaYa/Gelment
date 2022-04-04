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
            justify_content: "right",
            padding: "0.2rem",
            background_color: "white",
            width: "100%",
            height: "2rem",
            margin: "0.5rem",
            border_radius: "0.5rem",
            cursor: "pointer",
            div {
                align_items: "flex-start",
                width: "100%",
                height: "100%",
                border_radius: "0.5rem",
                background_color: "white",
                "{label}"
            }
            input {
                width: "100%",
                height: "100%",
                border: "none",
                text_align: "center",
                font_size: "1.5rem",
                font_weight: "bold",
                color: "black",
                background_color: "transparent",
                oninput: move |evt| {
                    println!("{:?}", evt);
                },
                value: "{value}",
            }
        }
    ))
}