use dioxus::{{prelude::*, events::MouseEvent}};
use crate::size::Size;
use crate::color::Color;

#[derive(Props)]
pub struct RateProps<'a> {
    #[props(default)]
    color: Color,

    #[props(default)]
    size: Size,

    #[props(default)]
    value: u8,

    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn Rate<'a>(cx: Scope<'a, RateProps<'a>>) -> Element {
    let names:Vec<u8> = vec![1, 2, 3, 4, 5];
    cx.render(rsx!(
        div {
            names.iter().map(move |item| {
                let width = cx.props.size.get_switch_width();
                let default_color = Color::Gray;
                let mut bg_color = cx.props.color.bg_color();
                let value = cx.props.value;
                if value < *item {
                    bg_color = default_color.bg_color();
                }
                rsx!(
                    input {
                        display: "inline-block",
                        width: "{width}",
                        height: "{width}",
                        background_color: "{bg_color}",
                        margin_right: ".2rem",
                        "",
                    }
                )
            })
        }
    ))
}