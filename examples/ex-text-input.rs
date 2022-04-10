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
                GelementTextInput { color: *color }
            )
        })
    ))
}


#[derive(Props, PartialEq)]
struct ColorProps {
    color: Color,
}

fn GelementTextInput (cx: Scope<ColorProps>) -> Element {
    cx.render(rsx!(
        div {
            div {
                TextInput {
                    value: "0"
                    size: Size::TINY,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                },
            }
            div {
                TextInput {
                    value: "0"
                    size: Size::SMALL,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                },
            }
            div {
                TextInput {
                    value: "0"
                    size: Size::MEDIUM,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                },
            }   
            div {
                TextInput {
                    value: "0"
                    size: Size::LARGE,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                }
            }
            div {
                TextInput {
                    value: "0"
                    size: Size::XLARGE,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                }
            }
            div {
                TextInput {
                    value: "0"
                    size: Size::XXLARGE,
                    maxlength: 5,
                    minlength: 2,
                    color: cx.props.color,
                }
            }         
        }
    ))
}