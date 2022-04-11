use dioxus::prelude::*;
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct SelectProps<'a> {
    #[props(default)]
    color: Color,
    #[props(default)]
    size: Size,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    value: &'a str,
}

pub fn Select<'a>(cx: Scope<'a, SelectProps<'a>>) -> Element {
    let color = cx.props.color.text_color();
    let font_size = cx.props.size.get_font_size();
    let disabled = cx.props.disabled;
    let value = cx.props.value;
    cx.render(rsx!(
        select {
            "type": "text",
            value: "{value}",
            color: "{color}",
            font_size: "{font_size}",
            disabled: "{disabled}",
            option { 
                value: "1",
                "Medium"
            }
            option { 
                value: "2",
                "Small"
            }
            option { 
                value: "3",
                "Large"
            }
        }
    ))
}