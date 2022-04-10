use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cxt: Scope) -> Element {
    let v_color: Vec<Color> = getColors();
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
                    max: 99,
                    min: 0,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::SMALL,
                    color: cx.props.color,
                    max: 99,
                    min: 0,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::MEDIUM,
                    color: cx.props.color,
                    max: 99,
                    min: 0,
                    label: "Number Input",
                },
            }   
            div {
                NumberInput {
                    value: 0,
                    size: Size::LARGE,
                    color: cx.props.color,
                    max: 99,
                    min: 0,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XLARGE,
                    color: cx.props.color,
                    max: 99,
                    min: 0,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XXLARGE,
                    color: cx.props.color,
                    max: 99,
                    min: 0,
                    label: "Number Input",
                }
            }         
        }
    ))
}