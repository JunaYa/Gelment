use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color = getColors();
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
            Rate {
                size: Size::TINY,
                color: cx.props.color,
                value: 2,
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
            Rate {
                size: Size::MEDIUM,
                color: cx.props.color,
                value: 2,
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
            Rate {
                size: Size::LARGE,
                color: cx.props.color,
                value: 2,
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
            Rate {
                size: Size::XLARGE,
                color: cx.props.color,
                value: 2,
                onclick: move |_| {
                    println!("GButton XLARGE");
                }
            }
            Rate {
                size: Size::XXLARGE,
                color: cx.props.color,
                value: 2,
                onclick: move |_| {
                    println!("GButton XXLARGE");
                }
            }
        }
    ))
}