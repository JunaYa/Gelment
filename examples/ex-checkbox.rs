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
        v_color.iter().map(|color| {
            rsx!(
                GCheckboxColors { color: *color }
            )
        })
    ))
}
#[derive(Props, PartialEq)]
struct BgColorProps {
    color: Color,
}
#[warn(non_snake_case)]
fn GCheckboxColors(cx: Scope<BgColorProps>) -> Element {
    cx.render(rsx!(
        div {
            Checkbox {
                disabled: false,
                size: Size::TINY,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-tiny",
                onclick: move |_| {
                    println!("GCheckbox TINY");
                }
            }
            Checkbox {
                disabled: false,
                size: Size::MEDIUM,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-medium",
                onclick: move |_| {
                    println!("GCheckbox MEDIUM");
                }
            }
            Checkbox {
                disabled: false,
                size: Size::LARGE,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-large",
                onclick: move |_| {
                    println!("GCheckbox LARGE");
                }
            }
            Checkbox {
                disabled: false,
                size: Size::XLARGE,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-xlarge",
                onclick: move |_| {
                    println!("GCheckbox XLARGE");
                }
            }
            Checkbox {
                disabled: true,
                size: Size::XXLARGE,
                label: "GCheckbox-xxlarge",
                color: cx.props.color,
                bg_color: cx.props.color,
                onclick: move |_| {
                    println!("GCheckbox XXLARGE");
                }
            }
        }
    ))
}