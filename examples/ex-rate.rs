use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            Rate {
                size: Size::TINY,
                color: Color::Gray,
                value: 2,
            }
        }
    ))
}