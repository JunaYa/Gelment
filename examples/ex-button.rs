use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
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
    cx.render(rsx! (
        button_disabled(),
        v_color.iter().map(|color| {
            rsx!(
                GButtonColors { color: *color }
            )
        })
    ))
}
fn button_disabled(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            Button {
                disabled: true,
                size: Size::TINY,
                text: "GButton-tiny",
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
            Button {
                disabled: true,
                size: Size::MEDIUM,
                text: "GButton-medium",
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
            Button {
                disabled: true,
                size: Size::LARGE,
                text: "GButton-large",
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
            Button {
                disabled: true,
                size: Size::XLARGE,
                text: "GButton-xlarge",
                onclick: move |_| {
                    println!("GButton XLARGE");
                }
            }
            Button {
                disabled: true,
                size: Size::XXLARGE,
                text: "GButton-xxlarge",
                onclick: move |_| {
                    println!("GButton XXLARGE");
                }
            }
        }
        
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
                disabled: true,
                size: Size::TINY,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-tiny",
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
            Button {
                disabled: true,
                size: Size::MEDIUM,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-medium",
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
            Button {
                disabled: true,
                size: Size::LARGE,
                color: Color::White,
                bg_color: cx.props.color,
                text: "GButton-large",
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
            Button {
                disabled: true,
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