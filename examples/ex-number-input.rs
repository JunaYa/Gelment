use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cxt: Scope) -> Element {
    let v_color: Vec<Color> = vec![
        Color::Transparent,
        Color::Black,
        Color::White,
        Color::Slate,
        Color::Gray,
        Color::Zinc,
        Color::Neutral,
        Color::Stone,
        Color::Red,
        Color::Orange,
        Color::Amber,
        Color::Yellow,
        Color::Lime,
        Color::Green,
        Color::Emerald,
        Color::Teal,
        Color::Cyan,
        Color::Sky,
        Color::Blue,
        Color::Indigo,
        Color::Violet,
        Color::Purple,
        Color::Fuchsia,
        Color::Pink,
        Color::Rose,
        Color::Default,
    ];
    cxt.render(rsx! (
        v_color.iter().map(|color| {
            rsx!(
                GelementNumberInput { color: *color }
            )
        })
    ))
}


#[derive(Props, PartialEq)]
struct ColorProps {
    color: Color,
}

fn GelementNumberInput (cx: Scope<ColorProps>) -> Element {
    cx.render(rsx!(
        div {
            div {
                NumberInput {
                    value: 0,
                    size: Size::TINY,
                    color: cx.props.color,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::SMALL,
                    color: cx.props.color,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::MEDIUM,
                    color: cx.props.color,
                    label: "Number Input",
                },
            }   
            div {
                NumberInput {
                    value: 0,
                    size: Size::LARGE,
                    color: cx.props.color,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XLARGE,
                    color: cx.props.color,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XXLARGE,
                    color: cx.props.color,
                    label: "Number Input",
                }
            }         
        }
    ))
}