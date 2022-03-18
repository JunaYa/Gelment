use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color};


fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        Radio{
            onclick: move |evt| {
                println!("radio click")
            }
        }
    ))
}