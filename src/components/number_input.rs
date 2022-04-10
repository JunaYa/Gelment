use dioxus::{prelude::*};
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct NumberInputProps<'a> {

    #[props(default)]
    pub value: i32,

    #[props(default)]
    pub size: Size,

    #[props(default)]
    pub color: Color,

    #[props(default)]
    pub max: i32,

    #[props(default)]
    pub min: i32,

    #[props(default)]
    pub label: &'a str,
}

pub fn NumberInput<'a>(cx: Scope<'a, NumberInputProps<'a>>) -> Element {
    let value = cx.props.value;
    let label = cx.props.label;
    let size = cx.props.size.get_font_size();
    let max = cx.props.max;
    let min = cx.props.min;
    let text_color = cx.props.color.text_color();
    let bg_color = cx.props.color.bg_color();
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
            cursor: "pointer",
            div {
                align_items: "flex-start",
                border_radius: "0.5rem",
                margin_right: "0.5rem",
                font_size: "{size}",
                color: "{text_color}",
                "{label}"
            }
            input {
                border: "none",
                text_align: "center",
                font_size: "1.5rem",
                font_weight: "bold",
                font_size: "{size}",
                color: "{text_color}",
                text_align: "center",
                max: "{max}",
                min: "{min}",
                oninput: move |evt| {
                    println!("{:?}", evt);
                },
                value: "{value}",
            }
        }
    ))
}