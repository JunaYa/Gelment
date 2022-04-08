use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};


fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color: Vec<Color> = getColors();
    cx.render(rsx!(
        v_color.iter().map(|color| {
            rsx!(
                div {
                    Radio{
                        checked: false,
                        color: *color,
                        size: Size::TINY,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                    Radio{
                        checked: true,
                        color: *color,
                        size: Size::SMALL,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                    Radio{
                        checked: true,
                        color: *color,
                        size: Size::MEDIUM,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                    Radio{
                        checked: true,
                        color: *color,
                        size: Size::LARGE,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                    Radio{
                        checked: true,
                        color: *color,
                        size: Size::XLARGE,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                    Radio{
                        disabled: true,
                        checked: false,
                        color: *color,
                        size: Size::XXLARGE,
                        onclick: move |_| {
                            println!("radio click")
                        }
                    }
                }
            )
        })
    ))
}