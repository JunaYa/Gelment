use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let vec = vec!["none", "dotted", "inset", "dashed", "solid", "double"];
    cx.render(rsx!(
        vec.iter().map(move |border_style| {
            rsx!(
                div {
                    Divider {
                        color: Color::Gray,
                        size: Size::TINY,
                        border_style: *border_style
                    }
                }
            )
        })
    ))
}