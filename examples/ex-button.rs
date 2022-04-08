use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color: Vec<Color> = getColors();
    cx.render(rsx! (
        v_color.iter().map(|color| {
            rsx!(
                GButtonColors { color: *color }
            )
        })
    ))
}
#[derive(Props, PartialEq)]
struct BgColorProps {
    color: Color,
}
#[warn(non_snake_case)]
fn GButtonColors(cx: Scope<BgColorProps>) -> Element {
    cx.render(rsx!(
        div {
            Button {
                disabled: false,
                size: Size::TINY,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-tiny",
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
            Button {
                disabled: false,
                size: Size::MEDIUM,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-medium",
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
            Button {
                disabled: false,
                size: Size::LARGE,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-large",
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
            Button {
                disabled: false,
                size: Size::XLARGE,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-xlarge",
                onclick: move |_| {
                    println!("GButton XLARGE");
                }
            }
            Button {
                disabled: true,
                size: Size::XXLARGE,
                text: "GButton-xxlarge",
                color: Color::White,
                bg_color: cx.props.color,
                onclick: move |_| {
                    println!("GButton XXLARGE");
                }
            }
        }
    ))
}