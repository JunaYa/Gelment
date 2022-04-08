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
    let names = [1, 2, 3, 4, 5];
    cx.render(rsx!(
        div {
            names.iter().map(move |item| {
                let width = cx.props.size.get_switch_width();
                let bg_color = cx.props.color.bg_color();
                rsx!(
                    div {
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