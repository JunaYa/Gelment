use dioxus::prelude::*;
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct RateProps {
    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,

    #[props(default)]
    value: u8,
}

pub fn Rate(cx: Scope<RateProps>) -> Element {
    cx.render(rsx!{
        div {
            for i in 0..cx.props.value {
                Star {
                    name: "star",
                    size: cx.props.size,
                    color: cx.props.color,
                }
            }
            for i in cx.props.value..5 {
                Star {
                    name: "star-outline",
                    size: cx.props.size,
                    color: cx.props.color,
                }
            }
        }
    })
}